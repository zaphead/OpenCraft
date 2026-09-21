use crate::craft::craft_result;
use crate::items::{item_max, merge};
use world::Stack;

pub const HOTBAR: usize = 9;
pub const MAIN: usize = 27;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowKind {
    Inventory,
    CraftingTable,
    Chest,
    Etch,
    Brew,
    Trade,
    Vault,
}

pub struct Window {
    pub kind: WindowKind,
    pub hotbar: [Stack; HOTBAR],
    pub main: [Stack; MAIN],
    pub craft: [Stack; 9],
    pub result: Stack,
    pub chest: [Stack; 27],
    pub cursor: Stack,
    pub selected: u8,
}

impl Window {
    pub fn player(hotbar: [Stack; HOTBAR], main: [Stack; MAIN], selected: u8) -> Self {
        Self {
            kind: WindowKind::Inventory,
            hotbar,
            main,
            craft: [None; 9],
            result: None,
            chest: [None; 27],
            cursor: None,
            selected,
        }
    }

    pub fn refresh_result(&mut self) {
        let w = match self.kind {
            WindowKind::Inventory => 2,
            WindowKind::CraftingTable => 3,
            WindowKind::Brew => {
                self.result = crate::brew(&self.craft[..3]);
                return;
            }
            WindowKind::Chest | WindowKind::Vault | WindowKind::Etch | WindowKind::Trade => {
                self.result = None;
                return;
            }
        };
        let n = w * w;
        self.result = craft_result(&self.craft[..n], w);
    }
}

enum Loc {
    Hotbar(usize),
    Main(usize),
    Craft(usize),
    Chest(usize),
    Result,
}

fn loc(kind: WindowKind, slot: u16) -> Option<Loc> {
    let m = layout(kind);
    if slot >= m.hotbar && slot < m.hotbar + HOTBAR as u16 {
        return Some(Loc::Hotbar((slot - m.hotbar) as usize));
    }
    if slot >= m.main && slot < m.main + MAIN as u16 {
        return Some(Loc::Main((slot - m.main) as usize));
    }
    if let Some(base) = m.chest {
        if slot >= base && slot < base + 27 {
            return Some(Loc::Chest((slot - base) as usize));
        }
    }
    if m.craft_n > 0 && slot >= m.craft && slot < m.craft + m.craft_n {
        return Some(Loc::Craft((slot - m.craft) as usize));
    }
    if slot == m.result {
        return Some(Loc::Result);
    }
    None
}

pub fn layout(kind: WindowKind) -> SlotMap {
    match kind {
        WindowKind::Inventory => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 36,
            craft_n: 4,
            result: 40,
            chest: None,
        },
        WindowKind::CraftingTable => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 36,
            craft_n: 9,
            result: 45,
            chest: None,
        },
        WindowKind::Chest | WindowKind::Vault => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 0,
            craft_n: 0,
            result: u16::MAX,
            chest: Some(36),
        },
        WindowKind::Etch => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 36,
            craft_n: 1,
            result: u16::MAX,
            chest: None,
        },
        WindowKind::Brew => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 36,
            craft_n: 3,
            result: 39,
            chest: None,
        },
        WindowKind::Trade => SlotMap {
            hotbar: 0,
            main: 9,
            craft: 36,
            craft_n: 3,
            result: u16::MAX,
            chest: None,
        },
    }
}

#[derive(Clone, Copy)]
pub struct SlotMap {
    pub hotbar: u16,
    pub main: u16,
    pub craft: u16,
    pub craft_n: u16,
    pub result: u16,
    pub chest: Option<u16>,
}

pub fn window_len(kind: WindowKind) -> usize {
    let m = layout(kind);
    let mut end = (m.hotbar as usize + HOTBAR).max(m.main as usize + MAIN);
    if m.craft_n > 0 {
        end = end.max(m.craft as usize + m.craft_n as usize);
    }
    if m.result != u16::MAX {
        end = end.max(m.result as usize + 1);
    }
    if let Some(c) = m.chest {
        end = end.max(c as usize + 27);
    }
    end
}

pub fn slot_at(w: &Window, slot: u16) -> Stack {
    match loc(w.kind, slot) {
        Some(Loc::Hotbar(i)) => w.hotbar[i],
        Some(Loc::Main(i)) => w.main[i],
        Some(Loc::Craft(i)) => w.craft[i],
        Some(Loc::Chest(i)) => w.chest[i],
        Some(Loc::Result) => w.result,
        None => None,
    }
}

fn get_mut<'a>(w: &'a mut Window, slot: u16) -> Option<&'a mut Stack> {
    match loc(w.kind, slot) {
        Some(Loc::Hotbar(i)) => Some(&mut w.hotbar[i]),
        Some(Loc::Main(i)) => Some(&mut w.main[i]),
        Some(Loc::Craft(i)) => Some(&mut w.craft[i]),
        Some(Loc::Chest(i)) => Some(&mut w.chest[i]),
        Some(Loc::Result) | None => None,
    }
}

pub fn click_slot(w: &mut Window, slot: u16, button: u8, shift: bool) {
    if w.kind == WindowKind::Trade {
        trade_buy(w, slot);
        return;
    }
    let m = layout(w.kind);
    if slot == m.result {
        take_result(w, shift);
        w.refresh_result();
        return;
    }
    if shift {
        quick_move(w, slot);
        w.refresh_result();
        return;
    }
    let mut cursor = w.cursor;
    {
        let Some(stack) = get_mut(w, slot) else {
            return;
        };
        match button {
            0 => left(stack, &mut cursor),
            1 => right(stack, &mut cursor),
            _ => {}
        }
    }
    w.cursor = cursor;
    w.refresh_result();
}

fn left(slot: &mut Stack, cursor: &mut Stack) {
    match (*slot, *cursor) {
        (None, None) => {}
        (Some(_), None) => {
            *cursor = *slot;
            *slot = None;
        }
        (None, Some(_)) => {
            *slot = *cursor;
            *cursor = None;
        }
        (Some((id_a, n_a)), Some((id_b, n_b))) if id_a == id_b => {
            let max = item_max(id_a);
            let sum = n_a as u16 + n_b as u16;
            if sum <= max as u16 {
                *slot = Some((id_a, sum as u8));
                *cursor = None;
            } else {
                *slot = Some((id_a, max));
                *cursor = Some((id_b, (sum - max as u16) as u8));
            }
        }
        _ => std::mem::swap(slot, cursor),
    }
}

fn right(slot: &mut Stack, cursor: &mut Stack) {
    match (*slot, *cursor) {
        (None, None) => {}
        (Some((id, n)), None) => {
            let take = (n + 1) / 2;
            *cursor = Some((id, take));
            let left = n - take;
            *slot = if left == 0 { None } else { Some((id, left)) };
        }
        (None, Some((id, n))) => {
            *slot = Some((id, 1));
            *cursor = if n == 1 { None } else { Some((id, n - 1)) };
        }
        (Some((id_a, n_a)), Some((id_b, n_b))) if id_a == id_b => {
            let max = item_max(id_a);
            if n_a < max {
                *slot = Some((id_a, n_a + 1));
                *cursor = if n_b == 1 { None } else { Some((id_b, n_b - 1)) };
            }
        }
        _ => std::mem::swap(slot, cursor),
    }
}

fn take_result(w: &mut Window, shift: bool) {
    let Some(res) = w.result else {
        return;
    };
    if w.cursor.is_some() && w.cursor.map(|s| s.0) != Some(res.0) {
        return;
    }
    let times = if shift { 64 } else { 1 };
    for _ in 0..times {
        let Some(cur) = w.result else { break };
        if !can_fit(w, cur) && w.cursor.is_some() {
            break;
        }
        if !consume_one_craft(w) {
            break;
        }
        if let Some(leftover) = merge(&mut w.cursor, cur) {
            try_insert(w, leftover);
        }
        w.refresh_result();
        if w.result != Some(cur) && !shift {
            break;
        }
    }
}

fn can_fit(w: &Window, st: (u16, u8)) -> bool {
    if w.cursor.is_none() {
        return true;
    }
    if let Some((id, n)) = w.cursor {
        if id == st.0 && n < item_max(id) {
            return true;
        }
    }
    w.hotbar.iter().chain(w.main.iter()).any(|s| match s {
        None => true,
        Some((id, n)) if *id == st.0 && *n < item_max(*id) => true,
        _ => false,
    })
}

fn consume_one_craft(w: &mut Window) -> bool {
    let n = match w.kind {
        WindowKind::Inventory => 4,
        WindowKind::CraftingTable => 9,
        WindowKind::Brew => 3,
        WindowKind::Chest | WindowKind::Vault | WindowKind::Etch | WindowKind::Trade => return false,
    };
    for i in 0..n {
        if let Some((id, c)) = w.craft[i] {
            if c <= 1 {
                w.craft[i] = None;
            } else {
                w.craft[i] = Some((id, c - 1));
            }
        }
    }
    true
}

fn try_insert(w: &mut Window, mut st: (u16, u8)) {
    for slot in w.hotbar.iter_mut().chain(w.main.iter_mut()) {
        if let Some(rest) = merge(slot, st) {
            st = rest;
        } else {
            return;
        }
    }
}

fn quick_move(w: &mut Window, slot: u16) {
    let m = layout(w.kind);
    let Some(taken) = get_mut(w, slot).and_then(|s| s.take()) else {
        return;
    };
    let player_end = m.main + MAIN as u16;
    let dest: Vec<u16> = if let Some(base) = m.chest {
        if slot < player_end {
            (base..base + 27).collect()
        } else {
            (m.hotbar..player_end).collect()
        }
    } else if slot < m.hotbar + HOTBAR as u16 {
        (m.main..player_end).collect()
    } else {
        (m.hotbar..m.hotbar + HOTBAR as u16)
            .chain(m.main..player_end)
            .filter(|i| *i != slot)
            .collect()
    };
    let mut left = Some(taken);
    for d in dest {
        let Some(st) = left else { break };
        if let Some(slot_ref) = get_mut(w, d) {
            left = merge(slot_ref, st);
        }
    }
    if let Some(rest) = left {
        if let Some(orig) = get_mut(w, slot) {
            *orig = Some(rest);
        }
    }
}

pub fn etch_tool(w: &mut Window) -> bool {
    let Some((id, _)) = w.craft[0] else { return false };
    let Some(next) = crate::etched(id) else { return false };
    w.craft[0] = Some((next, 1));
    true
}

fn trade_buy(w: &mut Window, slot: u16) {
    use crate::items::ItemId;
    let m = layout(WindowKind::Trade);
    let Some(idx) = slot.checked_sub(m.craft) else {
        // Player storage still moves, so a mis-click on the hotbar is a normal click.
        trade_move(w, slot);
        return;
    };
    if idx > 2 {
        return;
    }
    let (cost, n, give) = match idx {
        0 => (ItemId::HIDE, 8u8, ItemId::STAFF),
        1 => (ItemId::BEEF, 4, ItemId::KEY),
        _ => (ItemId::HONEYCOMB, 1, ItemId::GLOWCAP),
    };
    if !pay(w, cost, n) {
        return;
    }
    let mut give_stack = Some((give, 1u8));
    for slot in w.hotbar.iter_mut().chain(w.main.iter_mut()) {
        let Some(cur) = give_stack else { break };
        give_stack = merge(slot, cur);
    }
    if let Some(rest) = give_stack {
        let _ = merge(&mut w.cursor, rest);
    }
}

fn trade_move(w: &mut Window, slot: u16) {
    let mut cursor = w.cursor;
    {
        let Some(stack) = get_mut(w, slot) else { return };
        left(stack, &mut cursor);
    }
    w.cursor = cursor;
}

fn pay(w: &mut Window, id: u16, n: u8) -> bool {
    let have: u16 = w
        .hotbar
        .iter()
        .chain(w.main.iter())
        .filter_map(|s| s.and_then(|(i, c)| if i == id { Some(c as u16) } else { None }))
        .sum();
    if have < n as u16 {
        return false;
    }
    let mut left = n;
    for slot in w.hotbar.iter_mut().chain(w.main.iter_mut()) {
        if left == 0 {
            break;
        }
        let Some((sid, c)) = *slot else { continue };
        if sid != id {
            continue;
        }
        if c <= left {
            *slot = None;
            left -= c;
        } else {
            *slot = Some((sid, c - left));
            left = 0;
        }
    }
    true
}

pub fn collect_into(hotbar: &mut [Stack; 9], main: &mut [Stack; 27], st: (u16, u8)) -> Option<(u16, u8)> {
    let mut left = Some(st);
    for slot in hotbar.iter_mut().chain(main.iter_mut()) {
        let Some(cur) = left else { break };
        left = merge(slot, cur);
    }
    left
}

pub fn drop_all(hotbar: &mut [Stack; 9], main: &mut [Stack; 27]) -> Vec<(u16, u8)> {
    let mut out = Vec::new();
    for s in hotbar.iter_mut().chain(main.iter_mut()) {
        if let Some(st) = s.take() {
            out.push(st);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks;

    #[test]
    fn left_click_pickup() {
        let mut w = Window::player([Some((blocks::DIRT, 8)), None, None, None, None, None, None, None, None], [None; 27], 0);
        click_slot(&mut w, 0, 0, false);
        assert!(w.hotbar[0].is_none());
        assert_eq!(w.cursor, Some((blocks::DIRT, 8)));
    }

    #[test]
    fn window_len_follows_layout() {
        assert_eq!(window_len(WindowKind::Inventory), 41);
        assert_eq!(window_len(WindowKind::CraftingTable), 46);
        assert_eq!(window_len(WindowKind::Chest), 63);
    }
}
