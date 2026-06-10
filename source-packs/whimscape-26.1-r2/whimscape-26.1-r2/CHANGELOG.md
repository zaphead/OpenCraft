### **Whimscape 26.1_r2** (2026 May 18)

- Added custom entity models:
    - Sheep baby
    - Temperate cow baby
- Added painting: "Fern"
- Fixed beds and signs for 26.2
- Fixed striped wolf textures having a wrong colored pixel
- Fixed redstone dust dot texture having a semitransparent errant pixel
- Changed armor trims to use the default trim palette size
- Changed the painting "Cavebird" slightly
- Changed stonecutter textures slightly
- Removed Enderscape and Illager Invasion armor trim compatibility assets (obsolete)
- Farmer's Delight:
    - Added block models:
        - Basket template (custom texture coordinates)
        - Gleaming salad (custom texture coordinates)
        - Some default models to keep compatibility with earlier versions of the mod
    - Added block textures:
        - Platter (rice roll medley)
        - Wooden basket
        - Gleaming salad
        - Rope fence
        - Rope fence gate
    - Added item textures:
        - Gleaming salad
        - Bowl of gleaming salad
        - Onion soup
    - Added cooking pot JEI widget texture
    - Fixed cabbage stages 0-2 not rendering
    - Fixed tomatoes on rope using default models
    - Fixed bamboo basket using default textures
    - Changed placeable food models/textures to better fit their new hitboxes
    - Changed basket textures slightly

________________________________________________________________

### **Whimscape 26.1_r1** (2026 Apr 10)

- Added golden dandelion block and item texture
- Added textures for 26.1 baby models
- Added Lunge enchanted book
- Fixed textures/entity folder structure and filenames for 26.1
- Fixed held minecart items to refer to the new location of minecart textures
- Fixed CEM of cold cow, warm chicken and warm pig
- Fixed mooshroom textures missing ears when using default models
- Changed CEM:
    - Moved custom parts of cow and pig into their default texture files
- Changed entity textures:
    - Rabbits redesigned to fit the new model in 26.1
    - Cats and ocelot touched up
    - Camel saddle removed from main texture
    - Temperate chicken beak and foot color adjusted
    - Horses, donkey, mule:
        - Adjusted details and shading
        - Changed white and black horse colors
        - Removed saddles from textures
        - Redesigned the "black dots" markings
    - Stray overlay color adjusted
    - Squid color changed
    - Piglins tweaked
    - Wolves tweaked
- Changed item textures:
    - Squid spawn egg
- Changed the blocks atlas: added entity/minecart directory, removed single minecart entries
- Renamed grass models and textures to short_grass
- Removed custom entity models of baby mobs
- Removed overlay folders of past pack formats
- Removed redundant files from the main assets folder:
    - Block model files identical to the default files
    - Spawn egg item models and item definitions
    - blockstates/chain.json
    - blockstates/grass.json
    - blockstates/grass_path.json
    - blockstates/wooden_door.json
    - models/block/grass.json
    - models/item/chain.json
    - models/item/grass.json
    - models/item/clock_<00-15>.json
    - textures/entity/pig/pig.png
    - textures/entity/pig/pig_saddle.png
    - textures/entity/llama/decor folder
    - textures/entity/sheep/sheep_fur.png
    - textures/entity/strider/strider_saddle.png
    - textures/entity/wolf/wolf_armor.png
    - textures/entity/wolf/wolf_armor_overlay.png
    - textures/entity/chicken.png
    - textures/entity/elytra.png
    - textures/environment/moon_phases.png
    - textures/environment/sun.png
    - textures/map/map_icons.png
    - textures/misc/enchanted_glint_entity.png
    - textures/models folder
    - textures/trims/models folder
    - textures/item/keys folder (previously used by Dungeons and Taverns)

________________________________________________________________

### **Whimscape 1.20.2-1.21.11_r2** (2026 Jan 17)

- Added item textures:
    - Nautilus armor variants
    - Nautilus spawn egg
    - Zombie nautilus spawn egg
- Added entity textures:
    - Baby nautilus
    - Nautilus
    - Zombie nautilus
    - Coral zombie nautilus
    - Nautilus saddle
    - Nautilus armor variants
- Added GUI textures:
    - sprites/container/slot/nautilus_armor
    - sprites/container/slot/nautilus_armor_inventory
- Added mod support:
    - Trade Cycling button texture
    - Chat Patches GUI textures
    - Cloth Config GUI textures
    - YetAnotherConfigLib color picker texture
- Fixed JEI textures
- Fixed loading error with the Enderscape mod
- Changed item textures:
    - Breeze spawn egg
    - Nautilus shell
- Changed block models:
    - Moved the templates for bars, chains, fences, gates, lanterns and torches to a subfolder to avoid broken textures when a mod uses the original templates

________________________________________________________________

### **Whimscape 1.20.2-1.21.11_r1** (2025 Dec 9)

- Added item textures:
    - Spears
    - Spawn eggs: camel husk, parched
    - Leather horse armor overlay
    - Backstabbing enchanted book for Farmer's Delight (1.21.5+)
    - Charts and missing keys for Dungeons and Taverns
- Added entity textures:
    - Camel husk, camel husk saddle
    - Parched, parched_e
    - Horse armor: netherite, leather overlay
- Added paintings: backyard, orb
- Added breath_of_the_nautilus effect icon
- Added LabPBR emissive textures for copper lanterns to improve compatibility with Complementary Shaders IPBR+
- Added GUI textures:
    - container/nautilus
    - sprites/container/inventory/effect_background
    - sprites/container/inventory/effect_background_ambient
    - sprites/container/slot/spear
- Added .mcmeta files with mipmap_strategy/alpha_cutoff_bias fields for some blocks
- Fixed sun and moon texture paths (1.21.11)
- Fixed minecart with chest item model (1.21.11)
    - Added entity/minecart_chest.png
    - Added entity/minecart_chest to the blocks atlas
    - Removed from textures/item: chest_minecart_back.png, chest_minecart_front.png, chest_minecart_side.png and chest_minecart_top.png
- Changed item textures:
    - Horse armor: redesigned
    - Diamond items, lapis lazuli: changed outline color slightly
- Changed entity textures:
    - Horse armor and saddles
    - Husk: color tweak
    - Bogged, skeleton and stray: new face design
- Changed paintings: burning_skull, donkey_kong, pointer, skeleton, sunset, tides, unpacked, void
- Changed effect icons: bad_omen, darkness, oozing, raid_omen, saturation, trial_omen

________________________________________________________________

### **Whimscape 1.20.2-1.21.10_r1** (2025 Oct 14)

- Added assets for the new features in Minecraft 1.21.9
- Added textures for Farmer's Delight:
    - Pale oak cabinet block textures
    - Copper knife item texture
    - GUI sprites textures
- Added end portal texture
- Added dialog warning button textures
- Fixed custom sheep model textures Z-fighting when colored and sheared
- Fixed chains for 1.21.9+
- Fixed an incorrect color on exposed copper bulbs and weathered copper grate
- Fixed gui/container/villager.png result slot position for 1.21.9+
- Fixed potted crimson fungus and potted warped fungus missing their glowing parts in 1.21.2+
- Fixed Dungeons and Taverns asset location
- Changed block textures:
    - Lightning rod tweaked
    - chain.png renamed to iron_chain.png
    - Waxed block items' glint animation quickened
    - Iron bars texture layout changed
- Changed item textures:
    - Brewing stand
    - Chain changed slightly and renamed to iron_chain
    - Waxed copper doors
- Changed entity textures:
    - Spider custom models' leg textures tweaked
    - Allay eyes lowered by a pixel
    - Breeze_wind texture brightened slightly
- Changed copper trim colors to match other copper
- Changed pack overlay folder structure to align with the format numbers of full releases
    - format_18-19 is now format_18
    - format_18-27 is now format_18-22
    - Consolidated format_18-43 into format_18-42
- Renamed block model template_lantern_hanging to template_hanging_lantern
- Removed redundant block model files: lantern, lantern_hanging, soul_lantern, soul_lantern_hanging

________________________________________________________________

### **Whimscape 1.20.2-1.21.7_r1** (2025 Jun 30)

- Added LabPBR emissive textures for improved compatibility with the IPBR+ mode in Complementary Shaders
    - Enable these shader settings in Materials > IntegratedPBR+ Materials > Other IPBR+ Features:
        - IPBR+ Emissive Mode: labPBR > IPBR+
        - IPBR+ Compatibility Mode: On
- Added rails emissive texture for OptiFine/Continuity (1.20.2-1.21.1)
- Added "Lava Chicken" music disc texture
- Fixed the broken elytra item using the default texture in recent game versions
- Changed fire and campfire fire textures: removed darkest shade

________________________________________________________________

### **Whimscape 1.20.2-1.21.6_r1** (2025 Jun 17)

- Added enchanted book variants via an item model definition instead of CIT (1.21.5+)
- Added painting: passage
- Added happy ghast harness and rope textures
- Added dried ghast textures
- Added item textures:
    - Harnesses
    - Happy ghast spawn egg
    - "Tears" music disc
- Added GUI textures:
    - Locator bar background, arrows, dots
    - sprites/toast/now_playing
    - sprites/icon/music_notes
    - Pale oak hanging sign
- Fixed missing textures when using platinum armor trims in the Illager Invasion mod
- Fixed chorus flower not using its custom item model in recent game versions
- Changed ghastling texture: tweaked tentacles
- Changed lead knot top face texture to match the sides
- Changed light blue and yellow bundles: small outline color tweaks
- Changed enchanted book texture
- Removed enchanted book CIT (1.21.5+)

________________________________________________________________

### **Whimscape 1.20.2-1.21.5_r3** (2025 May 20)

- Added paintings: pond, sunflowers
- Added item sprites for boats, minecarts, lanterns, comparator, repeater, brewing stand, amethyst buds/cluster, pointed dripstone and iron bars (1.21.4+)
    - Held and dropped items are still shown as 3D models
    - In game versions before 1.21.4, the 3D models are still shown in every display context
- Added GUI textures: tab and text field widgets
- Added entity textures: happy ghast, ghastling
- Added dormant creaking heart block textures (1.21.5+)
- Added light_emission fields to many block model elements, replacing OptiFine/Continuity emissive textures (1.21.2+)
- Added soul lantern emissive texture for OptiFine/Continuity (1.20.2-1.21.1)
- Added waxed block item textures with a glint animation
- Fixed EMI mod support: added a missing sprite to widgets.png
- Fixed active/awake creaking heart block texture (1.21.5+)
- Changed GUI:
    - Button disabled textures
    - Header and footer separator textures
    - gui/sprites/trial_available texture
- Changed block and item models:
    - Brewing stand bottles
    - Amethyst buds/cluster display tweaks
    - Lantern display tweaks
    - Replaced negative-sized elements in cauldrons, hoppers, open barrel and outlined blocks
    - Moved shield idle position in first person view to be less intrusive and better match its third person position
    - Waxed blocks: removed outline from items
- Changed block textures:
    - Brewing stand bottles
    - End portal frame eye
    - Heavy core: changed top and bottom
    - Lantern emissive texture tweak for OptiFine/Continuity (1.20.2-1.21.1)
    - Sunflower
    - Torch flame animations
    - Composter, note block, jukebox, pitcher crop and scaffolding top: slight single-color tweak
- Changed item textures:
    - Chorus fruits
    - Cookie
    - Buckets
    - Ominous bottle
    - Sunflower
    - Torch flame animations
    - Minor tweaks to some other items
- Changed entity textures: tweaks to cow, ghast, shield and slime
- Changed paintings: earth, wind, fire, water
- Removed unused cauldron block models

________________________________________________________________

### **Whimscape 1.20.2-1.21.5_r2** (2025 Apr 20)

- Added mod support: TrashSlot
- Added bogged custom entity model with 3D mushrooms
- Added grass block models to only rotate the top texture
- Added bamboo gate models and fixed the texture
- Added axolotl bucket item variants for 1.21.5 (no CIT required)
- Added CIT for enchanted books: breach, density and wind burst (1.20.5+)
- Added animation to campfire item textures
- Fixed missing, redundant or incorrect texture and parent references in many models
- Fixed warnings about unused frames in conduit wind sprites by cropping them out
- Fixed small textures in the Waystones mod limiting mip level from 4 to 3
- Fixed CIT for axolotl buckets in 1.20.5-1.21.4
- Fixed CIT for sweeping edge enchanted book in 1.20.5+
- Fixed warm chicken baby and warm pig appearing as temperate variants when using EMF custom models
- Changed all pig variants' custom models and textures
- Changed slime entity texture slightly (missed in previous changelog)
- Changed conduit wind animation frametime from 3 to 1
- Changed fence and gate models: separated from default templates for mod compatibility
- Changed item textures: all books and smithing templates, spider eyes, egg, gold ingot
- Removed unmodified moss_template.png from Waystones mod assets
- Removed mod support: dorianpb's Custom Entity Models

________________________________________________________________

### **Whimscape 1.20.2-1.21.5_r1** (2025 Apr 12)

- Added mod support: Waystones
- Added paintings: bouquet, cavebird
- Added block and item textures for bush, cactus flower, dry grasses, firefly bush, leaf litter and wildflowers
- Added cold and warm variants for chicken, cow and pig
- Added sheep wool and undercoat textures
- Added blue and brown egg item textures
- Added leaf particle textures
- Added firefly particle texture
- Added cherry leaves variant texture
- Fixed mooshrooms and temperate cow/pig/chicken for 1.21.5
- Fixed armor enchantment glint for 1.21.5
- Fixed entity saddle textures for 1.21.5
- Fixed textures/gui/sprites/container/slot.png to better match GUI panels using it
- Fixed Fabric creative inventory page button texture for 1.21.1, 1.21.4 and 1.21.5
- Changed chicken texture
- Changed pig texture and custom model
- Changed lodestone side texture for 1.21.5 to match the new crafting recipe
- Changed egg item texture
- Changed pufferfish item texture colors slightly
- Changed dead bush block texture slightly
- Changed cherry leaves texture slightly
- Changed donkey, mule and zombie horse spawn egg textures: 1-pixel tweak

________________________________________________________________

### **Whimscape 1.20.2-1.21.4_r2** (2024 Dec 21)

- Added tuff brick wall and polished tuff wall models and textures
- Added matching inventory models for wall blocks with custom texture mapping
- Added slot highlight background texture
- Fixed clock item for 1.21.4
- Fixed wolf armor head tilting on its own
- Changed closed eyeblossom block and item textures slightly
- Removed textures/block/sand_s.png
- Removed blockstates/resin_brick_wall.json

________________________________________________________________

### **Whimscape 1.20.2-1.21.4_r1** (2024 Dec 3)

- Added creaking entity textures
- Added resin block, resin clump, resin bricks and chiseled resin bricks block textures
- Added resin brick wall models and textures
- Added resin brick and resin clump item textures
- Added pale oak leaf particle textures
- Added eyeblossom block and item textures
- Added resin trim palette
- Added painting: tides
- Fixed magma cube texture for 1.21.4
- Fixed spawn eggs for 1.21.4
- Fixed infested and waxed block items for 1.21.4
- Fixed GUI sprites for 1.21.4: empty slots, toast/system, toast/tutorial, advancements/box_obtained, advancements/box_unobtained
- Fixed two pixels in the bolt armor trim not matching the trim palette
- Changed pale oak door and sign item colors slightly
- Changed active creaking heart textures: color tweak
- Changed chestplate armor trim item overlay slightly
- Changed spawn eggs: tweaks to creaking, mooshroom and witch

________________________________________________________________

### **Whimscape 1.20.2-1.21.3_r1** (2024 Oct 26)

- Added pale oak wood set textures and models
- Added pale moss textures and models
- Added pale oak log and leaf textures
- Added pale oak sapling textures and models
- Added creaking heart textures
- Added creaking spawn egg
- Added bordure indented and field masoned banner pattern item textures
- Added empty air bubble HUD texture
- Added mod support:
    - Dungeons and Taverns key item textures
    - Visuality particle textures
    - Mod Menu GUI textures
    - oωo library GUI textures
    - Entity Features config button textures
    - Roughly Enough Items (REI)
    - EMI
    - Fabric creative inventory page buttons texture
- Added missing Just Enough Items (JEI) textures
- Fixed errant pixel in fox and wolf spawn egg items
- Fixed tropical fish fin patterns being one-sided
- Changed some GUI elements
- Changed font glyphs: f, G, i, J, j, t, y and some Cyrillic characters
- Changed enchanted glint textures
- Changed many spawn egg items slightly
- Changed acacia leaves color slightly
- Changed grass and fern base texture colors: reduced saturation of darker shades
- Changed banner pattern item textures

________________________________________________________________

### **Whimscape 1.20.2-1.21.1_r1** (2024 Sep 30)

- Added paintings: baroque, changing, meditative, prairie_ride, unpacked
- Added bundle variant textures
- Added bundle GUI textures
- Added tooltip textures
- Added missing torch template block models
- Added comparator and repeater models (from 24w38a with UV tweaks)
- Added textures for comparator block subtraction mode
- Fixed equipment model texture locations in pack_format 37+
- Fixed bundle textures in pack_format 35+
- Fixed arrow and bee stinger entity textures in pack_format 35+
- Changed wall torch block model slightly
- Changed redstone torch block and item textures
- Changed comparator and repeater textures slightly
- Changed pitcher plant item texture slightly
- Changed bee spawn egg item texture: 1-pixel tweak
- Changed brown dye and brown candle item texture colors

________________________________________________________________

### **Whimscape 1.20.2-1.21_r2** (2024 Jul 10)

- Added bogged entity textures
- Added bogged spawn egg texture
- Added status effect icons: infested, oozing, weaving, wind_charged
- Added particle textures: infested, ominous_spawning, raid_omen, small_gust, trial_omen, trial_spawner_detection_ominous, vault_connection
- Added painting: humble
- Fixed moss carpet sides being fully opaque in 1.21
- Changed wind charge item texture
- Changed gust particle colors
- Changed wind charge projectile colors

________________________________________________________________

### **Whimscape 1.20.2-1.21_r1** (2024 Jun 15)

- Added item textures: mace, ominous bottle, ominous trial key, wind charge
- Added music discs: "Creator", "Creator (Music Box)", "Precipice"
- Added vault, ominous vault and ominous trial spawner textures
- Added bolt and flow armor trim textures
- Added raid omen and trial omen effect icons
- Fixed copper bulb texture names for lit and powered states
- Changed trial spawner textures
- Changed trial key texture
- Changed bad omen effect icon for 1.21
- Changed crafter north face and weathered copper bulbs: 2-pixel tweaks

________________________________________________________________

### **Whimscape 1.20.2-1.20.5_r1** (2024 Apr 24)

- Added wolf variant textures
- Added wolf armor overlay and crackiness textures
- Added spawn egg textures for armadillo and breeze
- Added breeze rod texture
- Added flow and bolt smithing template textures
- Added flow and guster banner and shield patterns
- Added flow, guster and scrape pottery textures
- Added GUI inworld_footer_separator, inworld_header_separator and inworld_menu_background
- Added gui/sprites/widget/scroller_background
- Added trial_chambers map icon
- Fixed map icons for resource pack format 30
- Changed wolf textures: original wolf is now lighter
- Changed pitcher plant colors
- Changed sea lantern texture
- Changed guardian and elder guardian textures

________________________________________________________________

### **Whimscape 1.20.2-1.20.4_r2** (2024 Mar 19)

- Added custom spawn egg item textures
- Added mod support:
    - AppleSkin
    - Just Enough Items
    - Farmer's Delight
- Added wall torch model with a holder
- Fixed menu GUI elements for resource pack format 28
- Fixed wind charge projectile texture
- Fixed dolphin flipper texture orientation
- Changed buttons and sliders to a simpler design
- Changed recipe book craftable toggles slightly
- Changed food HUD icons: reduced size for clean AppleSkin outlines
- Changed fuel progress textures for a cleaner look
- Changed pumpkins: cleanup and color adjustment
- Changed bee nest: partial redesign
- Changed honeycomb block and beehive honey color
- Changed beetroots and potatoes: shading tweak on the tuber
- Changed crimson and warped stem tops: made distinct from regular logs
- Changed cake block slightly
- Changed mossy stone brick wall tops slightly
- Changed brewing stand rod texture and added emissive map
- Changed vindicator pants color
- Changed farmland textures slightly
- Changed pumpkin pie item texture slightly
- Changed potions, experience bottle and dragon's breath bottle highlights
- Changed stone axe, stone hoe, stone shovel, wooden shovel and wooden axe slightly
- Changed cookie texture to be smaller
- Changed mutton and cooked mutton textures
- Changed bowl, stew and soup textures
- Changed bone and bone meal texture colors slightly

________________________________________________________________

### **Whimscape 1.20.2-1.20.4_r1** (2023 Dec 20)

- Added gust particle textures
- Added wind charge texture
- Added armadillo texture
- Added armadillo scute texture
- Added wolf armor textures
- Fixed turtle scute item texture name for snapshot 23w51a
- Fixed breeze eyes for snapshot 23w51a
- Changed the font's ampersand to be closer to the modern form
- Changed potion textures
- Changed dragon's breath texture
- Changed experience bottle texture
- Changed honey bottle texture
- Changed flower pot item texture
- Changed goat horn texture

________________________________________________________________

### **Whimscape 1.20.2-1.20.3_r1** (2023 Dec 5)

- Added models and textures for the new experimental features
- Added darkness effect icon texture
- Added textures/gui/sprites/widget/slot_frame.png
- Fixed bat texture for the new model in 1.20.3
- Fixed short grass in 1.20.3
- Fixed waxed slab outline missing down-facing texture
- Fixed button block textures being flipped vertically
- Changed infested and waxed blocks: slightly thinner item outlines
- Changed raw copper block texture slightly
- Changed tuff colors slightly
- Removed leftover realms folder

________________________________________________________________

### **Whimscape 1.20.2_r1** (2023 Sep 26)

- Added new icons in map_icons.png
- Fixed GUI for 1.20.2
- Changed structure_block_load and structure_block_save textures
- Changed rail item outlines slightly
- Changed gold and copper ingot textures for better distinction
- Changed pack_format to 18

________________________________________________________________

### **Whimscape 1.20_r3** (2023 Jun 16)

- Added paintings: earth, fire, water, wind
- Fixed pot patterns: missing plenty pattern and incorrect prize texture

________________________________________________________________

### **Whimscape 1.20_r2** (2023 Jun 11)

- Fixed OptiFine applying a tint to cherry leaves
- Fixed sniffer egg stage inconsistencies
- Changed jungle door and trapdoor design
- Changed iron door slightly to better show orientation
- Changed some other doors: minor tweaks

________________________________________________________________

### **Whimscape 1.20_r1** (2023 Jun 7)

- Added sniffer and sniffer egg textures
- Added pitcher plant and torchflower blocks and items
- Added armor trims and templates: host, raiser, shaper, silence, wayfinder
- Added decorated pot textures
- Added item textures: brush, pottery sherds, "Relic" music disc
- Added suspicious gravel
- Added calibrated sculk sensor model and textures
- Fixed menu logos and invite_icon for the new texture layouts in 1.20
- Changed armor trims: dune, sentry
- Changed some smithing templates
- Changed cherry particles
- Changed suspicious sand slightly
- Changed bamboo and cherry doors and trapdoors
- Changed bamboo block top texture
- Changed pack_format to 15

________________________________________________________________

### **Whimscape 1.19.4_r1** (2023 Mar 15)

- Added 1.20 data pack features: armor trims, cherry things, suspicious sand
- Added world creation screen textures
- Added the new separate GUI slider texture
- Fixed smithing GUI textures and tweaked legacy GUI
- Changed some armor to work better with armor trims
- Changed enchanted glint slightly
- Changed pack_format to 13

________________________________________________________________

### **Whimscape 1.19.3_r8** (2023 Mar 12)

- Added custom wither model and tweaked wither skull
- Added sand and red sand variant
- Added dorianpb/cem folder for Fabric CEM, removing animations that break it
- Fixed witch missing textures when using Fabric CEM
- Fixed slight color inconsistency in empty armor slot textures
- Changed amethyst blocks to better match the custom amethyst cluster model
- Changed some pink colors slightly and tweaked pink glazed terracotta pattern
- Changed anvil GUI icon

________________________________________________________________

### **Whimscape 1.19.3_r7** (2023 Feb 19)

- Added 1.20 data pack features: bamboo stuff, chiseled bookshelf, hanging signs and camel
- Fixed nether wart missing break particles
- Fixed rogue pixel in kelp texture
- Fixed missing pixels in angry bee with nectar custom model
- Fixed Z-fighting in fish fins
- Changed bamboo stalk to better match new bamboo blocks
- Changed wandering trader custom model: added 3D pouches
- Changed pillager, witch, wolf and horse_creamy textures slightly
- Changed item frame backgrounds
- Changed loom, smithing table and stonecutter textures slightly
- Changed azalea blocks slightly
- Changed netherite: colors, sword to match tools better
- Changed ancient debris colors slightly
- Changed many items with minor color adjustments

________________________________________________________________

### **Whimscape 1.19.3_r6** (2023 Feb 5)

- Added custom bee model
- Added custom witch model
- Added axolotl bucket textures for different colors via CIT
- Added custom colors: fog.end, sky.end, map.stone, map.water, xp orb
- Added obsidian variant
- Added wither armor effect texture
- Added emissive texture for strider eyes
- Fixed creative inventory alignment with tabs
- Changed ore blocks to better distinguish different ores
- Changed enderman and endermite textures
- Changed many mobs with small tweaks
- Changed obsidian blocks and entities: mostly color tweaks
- Changed many blocks with minor color adjustments
- Changed minecart shading slightly
- Changed minecart items to use the actual minecart texture
- Changed some item textures slightly
- Changed some armor textures slightly
- Changed main menu logo colors slightly

________________________________________________________________

### **Whimscape 1.19.3_r5** (2023 Jan 15)

- Added custom models for chorus flower and plant
- Fixed mossy stone brick slabs and stairs not using variant textures
- Fixed drowned flipped arm and leg
- Fixed wolf collar Z-fighting with custom entity models enabled
- Fixed glass pane CTM not connecting properly in corners
- Changed glass and stained glass CTM to enable innerSeams
- Changed ice, packed ice and frosted ice textures and added variants
- Changed lever texture
- Changed hopper texture to make direction clear from above
- Changed stonecutter textures
- Changed smoker bottom texture
- Changed various other blocks slightly
- Changed many entity textures with small tweaks
- Changed pack.png slightly

________________________________________________________________

### **Whimscape 1.19.3_r4** (2023 Jan 5)

- Added custom models for pumpkin and melon stems
- Fixed netherite armor still using old textures in 1.19.3 releases
- Fixed door side UV inconsistency
- Fixed custom door models causing mod compatibility issues
- Changed lectern textures slightly
- Changed cactus bottom texture
- Changed some green colors slightly
- Removed unused door model files

________________________________________________________________

### **Whimscape 1.19.3_r3** (2022 Dec 23)

- Added unique enchanted book textures
    - requires OptiFine, Options/Video Settings/Quality/Custom Items: ON
- Changed all book item textures slightly
- Changed experience bottle and dragon's breath item textures: added glow outlines

________________________________________________________________

### **Whimscape 1.19.3_r2** (2022 Dec 17)

- Added CTM for glass, stained glass, glass pane and tinted glass
    - You can delete a block's folder in assets\minecraft\optifine\ctm if you prefer no CTM, or turn it off in video settings
- Changed glass and tinted glass colors slightly
- Changed nether gold ore texture slightly
- Changed iron block and iron trapdoor slightly
- Changed netherite block, ancient debris and lodestone colors
- Changed netherite armor: partial redesign for a more sinister feel
- Changed various items (mostly slight color tweaks)

________________________________________________________________

### **Whimscape 1.19.3_r1** (2022 Dec 7)

- Added textures/gui/checkbox.png
- Added textures/gui/report_button.png
- Added outline to waxed copper block items
- Added jigsaw block textures
- Fixed vex textures for the new 1.19.3 model
- Fixed missing transparency in allay texture
- Fixed creative inventory textures for 1.19.3
- Fixed boat item textures missing from atlases/blocks.json
- Fixed door UV inconsistencies
- Changed mangrove wood colors slightly
- Changed bookshelf content colors slightly
- Changed stripped log top texture by one pixel
- Changed OptiFine fog0 colormap slightly
- Changed pack_format to 12
- Removed some unused model files

________________________________________________________________

### **Whimscape 1.19_r2** (2022 Oct 8)

- Added CHANGELOG.md
- Added missing toasts.png icons
- Added missing stripped mangrove log textures
- Changed mangrove wood block colors to match boats and signs
- Removed unused glowstone .mcmeta file

________________________________________________________________

### **Whimscape 1.19_r1** (2022 Sep 29)

- Initial release