pub mod globals {
    // 0x1
    pub const FIGHTER_KIND:          i32 = 0x2;
    pub const OBJECT_ID:             i32 = 0x3;
    // 0x4
    pub const MODULE_ACCESSOR:       i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC:      i32 = 0x7;
    pub const IS_STOPPING:           i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND:      i32 = 0xA;
    pub const STATUS_KIND:           i32 = 0xB;
    pub const STATUS_COUNT:          i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME:         i32 = 0xE;
    pub const CURRENT_FRAME2:        i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3:           i32 = 0x13;
    pub const SUB_STATUS2:           i32 = 0x14;
    pub const SUB_STATUS:            i32 = 0x15;
    pub const SITUATION_KIND:        i32 = 0x16;
    pub const PREV_SITUATION_KIND:   i32 = 0x17;
    pub const PREV_STATUS_FRAME:     i32 = 0x18;
    // 0x19
    pub const STICK_X:               i32 = 0x1A;
    pub const STICK_Y:               i32 = 0x1B;
    pub const FLICK_X:               i32 = 0x1C;
    pub const FLICK_Y:               i32 = 0x1D;
    pub const FLICK_Y_DIR:           i32 = 0x1E;
    pub const PAD_FLAG:              i32 = 0x1F;
    pub const CMD_CAT1:              i32 = 0x20;
    pub const CMD_CAT2:              i32 = 0x21;
    pub const CMD_CAT3:              i32 = 0x22;
    pub const CMD_CAT4:              i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK:         i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE:        i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const LEAVE_STOP_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

/*
WuBoy's VarModule Constant Overhaul!
The way our variable constants are labelled is changing.

Variables now have two categories:
INSTANCE, which persists until manually changed. Represented by 0x0XXX.
STATUS, which is automatically reset when the status changes. Represented by 0x1XXX.

In addition, there are two sub-categories.
Common, which is shared by every fighter. Represented by 0xX0XX.
Agent, which is specific to a certain fighter/agent. Represented by 0xX1XX.

This means for each combination, you have access to 256 common variables and 256 agent variables.
That's a LOT of space, and I don't anticipate it all gets used up with proper variable application.
*/

pub mod vars {
    pub mod common {
        pub mod instance {
            pub const HITSTUN_START: i32 = 0x0000;
            pub const IS_IN_HITSTUN: i32 = 0x0001;

            pub const CSTICK_OVERRIDE: i32 = 0x0002;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0003;

            pub const IS_TAP_JUMP: i32 = 0x0004;

            pub const OMNI_FLOAT: i32 = 0x0005;
            pub const AERIAL_NO_FLOAT: i32 = 0x0006;
            pub const FLOAT_PAUSE_AERIAL: i32 = 0x0007;

            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0008;
            pub const UP_SPECIAL_CANCEL: i32 = 0x0009;

            pub const JAB_DA_CHECKS: i32 = 0x000A;
            pub const TILT_CHECKS: i32 = 0x000B;
            pub const AERIAL_CHECKS: i32 = 0x000C;
            pub const SMASH_CHECKS: i32 = 0x000D;

            pub const SPECIAL_STALL: i32 = 0x000E;
            pub const SPECIAL_STALL_USED: i32 = 0x000F;

            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x0010;

            pub const DITCIT_SLIDING: i32 = 0x0011;

            pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 0x0012;

            pub const CAN_ESCAPE_TUMBLE: i32 = 0x0013;

            pub const SPECIAL_WALL_JUMP: i32 = 0x0014;

            pub const TETHER_HOGGED: i32 = 0x0015;

            pub const B_REVERSED: i32 = 0x0016; // Converted for now, but will likely get removed when B Reverse Reimplementation happens

            pub const TUMBLE_KB: i32 = 0x0017;

            pub const CAN_GLIDE_TOSS: i32 = 0x0019;

            pub const IS_MOTION_BASED_ATTACK: i32 = 0x001A;

            pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 0x001B;

            pub const ENABLE_WAVELAND_PLATDROP: i32 = 0x001C;

            pub const IS_DACUS: i32 = 0x001D;

            pub const IS_STICKY_WALK: i32 = 0x001E;
            pub const ENABLE_BOOST_RUN: i32 = 0x001F;

            pub const PERFECT_WAVEDASH: i32 = 0x0020;

            pub const JUMP_NEXT: i32 = 0x0021;

            pub const SHOULD_TRUMP_TETHER: i32 = 0x0022;

            pub const UP_SPECIAL_INTERRUPT: i32 = 0x0023; // Ness and Lucas use this
            pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x0024; // Ness and Lucas use this
            
            pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x0025; // Luigi, Ivysaur, and Young Link use this
            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x0026; // Mewtwo, Palutena, Sheik, and Zelda use this
            pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x003E; // Link and Mii Sword use this
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x004D; // Used by Kazuya and Sora

            pub const IS_LATE_PIVOT: i32 = 0x004E;
            pub const CAN_PERFECT_PIVOT: i32 = 0x004F;
            pub const IS_SMASH_TURN: i32 = 0x0050;

            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x0051;

            pub const IS_KNOCKDOWN_THROW: i32 = 0x0052;

            pub const IS_HEAVY_ATTACK: i32 = 0x0053;

            // ints

            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;

            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001; // Unironically why does this need to exist? We have WorkModule.

            pub const FLOAT_TIMER: i32 = 0x0002;
            pub const FLOAT_DURATION: i32 = 0x0003;
            pub const FLOAT_STYLE: i32 = 0x0004;

            pub const HITFALL_BUFFER: i32 = 0x0005;

            pub const JUMP_SQUAT_FRAME: i32 = 0x0006;

            pub const GIMMICK_TIMER: i32 = 0x0007;

            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0008;

            pub const CSTICK_LIFE: i32 = 0x0009;

            pub const AGT_USED_COUNTER: i32 = 0x000A;

            pub const CLIFF_XLU_FRAME: i32 = 0x000B;

            // floats

            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0000;

            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            /// This const is set in a fighter reset because the params used to calculate change depending on situation
            pub const JUMP_SPEED_RATIO: i32 = 0x0003;
            pub const DOUBLE_JUMP_FRAME: i32 = 0x0004;
            pub const GROUND_VEL: i32 = 0x0005; // Only ever gets set, goes effectively unused.
            pub const RAR_LENIENCY: i32 = 0x0006; // Only ever gets set, goes effectively unused.
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0007;
            pub const DOUBLE_JUMP_TIMER: i32 = 0x0008; // Only used by Lucas, and it's commented out, goes unused.
            pub const ROLL_SPEED: i32 = 0x0009;
            pub const LEDGE_POS: i32 = 0x000A;
            pub const LEDGE_POS_X: i32 = 0x000A;
            pub const LEDGE_POS_Y: i32 = 0x000B;
            pub const LEDGE_POS_Z: i32 = 0x000C;
            pub const GET_DIST_TO_FLOOR: i32 = 0x000D;
            pub const ECB_Y_OFFSETS: i32 = 0x000E;
            pub const CURR_DASH_SPEED: i32 = 0x000F;
            pub const MOONWALK_SPEED: i32 = 0x0010;
            pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x0011;
            pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x0012;
            pub const Y_POS: i32 = 0x0013;
            /// this multiplier can be set to a value between 0.1 and 3.0 to increase
            /// a character's jump speed max for momentum transfer (for meta quick, etc)
            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0014;
        }
        pub mod status {
            // flags

            pub const DISABLE_BACKDASH: i32 = 0x1000;
            pub const IS_MOONWALK: i32 = 0x1001;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1002;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1003;

            pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 0x1000;

            pub const SHOULD_WAVELAND: i32 = 0x1000;

            pub const IS_JAB_LOCK_ROLL: i32 = 0x1000;
            pub const IS_SPIKE: i32 = 0x1001;

            pub const SUICIDE_THROW_CAN_CLATTER: i32 = 0x1000;

            pub const ENABLE_UCF: i32 = 0x1000;

            // ints

            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;

            // floats

            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;

        }
    }

    pub mod bayonetta {
        pub mod instance {
            // flags
            pub const IS_NONSPECIAL_CANCEL:        i32 = 0x0100;
            pub const SHOULD_PRORATE_DAMAGE:       i32 = 0x0101;
            pub const IS_SPECIAL_S_CANCELED_INTO:  i32 = 0x0102;
            pub const IS_SPECIAL_HI_CANCELED_INTO: i32 = 0x0103;

            // ints
            pub const NUM_RECOVERY_RESOURCE_USED:         i32 = 0x0100;
            pub const NUM_SPECIAL_S_CANCEL_THIS_AIRTIME:  i32 = 0x0101;
            pub const NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME: i32 = 0x0102;
        }
        pub mod status {
            // flags
            pub const IS_BULLET_ARTS: i32 = 0x1100;
        }
    }

    pub mod brave {
        pub mod status {
            // flags
            pub const IS_CRITICAL_HIT: i32 = 0x1100;
        }
    }

    pub mod buddy {
        pub mod status {
            // flags
            pub const IS_BURY_DTHROW: i32 = 0x1100;
        }
    }

    pub mod chrom {
        pub mod instance {
            // flags
            pub use super::super::roy::instance::TRAIL_EFFECT;
        }
        pub mod status {
            // flags
            pub const SOARING_SLASH_HIT: i32 = 0x1100;
        }
    }

    pub mod demon {
        pub mod instance {
            // flags
            pub const SLAUGHTER_HIGH_KICK:      i32 = 0x0100;
            pub const DEVASTATOR:               i32 = 0x0101;
            pub const JAW_BREAKER:              i32 = 0x0102;
            pub const SLICING_BLADE:            i32 = 0x0103;
            pub const SPINNING_DEMON:           i32 = 0x0104;
            pub const LIGHTNING_SCREW_UPPERCUT: i32 = 0x0105;
            pub const TWIN_FANG_DOUBLE_KICK:    i32 = 0x0106;
        }
    }

    // Note: Terry starts his flags on 0xXX5X instead due to also using the shotos generic flags.
    pub mod dolly {
        pub mod instance {
            pub const SUPER_CANCEL: i32 = 0x0150;
        }
        pub mod status {
            // flags
            pub const IS_USE_FIRE_KICK:      i32 = 0x1150;
            pub const UNABLE_CANCEL_S3_DASH: i32 = 0x1151;
            pub const IS_CHAIN_CANCEL:       i32 = 0x1152;
            pub const IS_SHATTER_STRIKE:     i32 = 0x1153;
            pub const AIR_SPECIAL_F:         i32 = 0x1154;
        }
    }

    pub mod donkey {
        pub mod status {
            // flags
            pub const SPECIAL_CHECKS: i32 = 0x1100;
        }
    }

    pub mod duckhunt {
        pub mod instance {
            // int
            pub const GUNMAN_TIMER: i32 = 0x0100;
        }
    }

    pub mod rosetta {
        pub mod instance {
            // ints
            pub const COOLDOWN: i32 = 0x0100;
            pub const ROSA_X: i32 = 0x0101;
            pub const ROSA_Y: i32 = 0x0102;
            pub const TICO_X: i32 = 0x0103;
            pub const TICO_Y: i32 = 0x0104;
            pub const TICO_RAYCAST: i32 = 0x0106;
            pub const TICO_X_DIST: i32 = 0x0107;
            pub const TICO_Y_DIST: i32 = 0x0108;
			
			// flag
            pub const IS_TICO_DEAD: i32 = 0x0105;
        }
        pub mod status {
            // int
            /// Used for determining what luma does
            pub const INVIS_FRAMES: i32 = 0x1100;
        }
    }

    pub mod elight {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI_JUMP: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S:       i32 = 0x0101;
        }
        pub mod status {
            // ints
            /// This is used to determine how to end the SpecialHiJump status script
            pub const SPECIAL_HI_JUMP_RESERVE_ACTION: i32 = 0x1100;
        }

        // not IDs but symbolic consts
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1: i32 = 0x0;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2: i32 = 0x1;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_FALL:    i32 = 0x2;
    }

    pub mod ganon {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_N: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const FLOAT_ENABLE_ACTIONS: i32 = 0x1100;
            pub const FLOAT_FALL_SPEED_Y_INCREASE: i32 = 0x1101;
            pub const FLOAT_CANCEL: i32 = 0x1102;
            pub const FLOAT_GROUND_CHANGE_KINETIC: i32 = 0x1103;
        }

    }

    pub mod gaogaen {
        pub mod instance {
            // flags
            pub const IS_SPECIAL_S_ALTERNATE_GRAB: i32 = 0x0100;
            pub const IS_SPECIAL_S_GROUND_GRAB:    i32 = 0x0101;
            pub const IS_SPECIAL_S_AIR_GRAB:       i32 = 0x0102;

        }
        pub mod status {
            // flags
            pub const IS_HIT_SPECIAL_HI_RISE:       i32 = 0x1100;
            pub const IS_INPUT_CROSS_CHOP_CANCEL:   i32 = 0x1101;
            pub const SHOULD_CROSS_CHOP_DIVE_EARLY: i32 = 0x1102;

            // ints
            pub const SPECIAL_N_STRENGTH_LEVEL: i32 = 0x1100;

            // floats
            pub const ANGLE_GRAB_STICK_Y: i32 = 0x1100;
        }
    }

    pub mod ike {
        pub mod status {
            // flags
            pub const IS_QUICK_DRAW_INSTAKILL: i32 = 0x1100;
        }
    }

    pub mod inkling {
        pub mod status {
            // flags
            pub const IS_ENABLE_SPECIAL_S_JUMP_EARLY_CANCEL: i32 = 0x1100;
        }
    }

    pub mod kamui {
        pub mod status {
            // flags
            pub const IS_CHARGE_FINISHED: i32 = 0x1100;
            pub const CHARGE_ATTACK_LEVEL: i32 = 0x1101;
        }
    }

    pub mod kirby {
        pub mod status {
            // flags
            pub const FINAL_CUTTER_HIT: i32 = 0x1100;
        }
    }

    // pub mod koopa {
    //     pub mod instance {
    //         // flags
    //         pub use super::super::mario::instance::NOKNOK_SHELL;
    //     }
    // }

    pub mod lucario {
        pub mod status {
            // ints
            pub const FORCE_PALM_ROT_ANGLE: i32 = 0x1100;
        }
    }
    pub mod lucas {

        pub mod instance {
            // flag
            pub const SPECIAL_N_OFFENSE_UP_ACTIVE: i32 = 0x0100;
            pub const SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF: i32 = 0x0101;
            pub const SPECIAL_N_OFFENSE_UP_INIT: i32 = 0x0102;
            pub const ATTACK_S4_ANGLE_DOWN: i32 = 0x0103;
            pub const ATTACK_S4_ANGLE_UP: i32 = 0x0104;

            // int
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1: i32 = 0x0100;
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2: i32 = 0x0101;
            pub const SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL: i32 = 0x0102;
        }

        pub mod status {
            // flag
            pub const SPECIAL_HI_ATTACK_IS_TOUCH_WALL: i32 = 0x1100;
            pub const SPECIAL_HI_ATTACK_IS_LEAVE_WALL: i32 = 0x1101;
            pub const SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH: i32 = 0x1102;
            pub const SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM: i32 = 0x1103;

            // float
            /// Holds the initial y velocity you have during up b to compare once you've touched a wall to reflect your speed the other direction if it changes
            pub const SPECIAL_HI_ATTACK_Y_MOMENTUM: i32 = 0x1100;
            pub const SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM: i32 = 0x1101;
            pub const SPECIAL_HI_ATTACK_Y_DIRECTION: i32 = 0x1102;
            pub const SPECIAL_HI_ATTACK_X_MOMENTUM: i32 = 0x1103;
            pub const SPECIAL_HI_ATTACK_X_INIT_MOMENTUM: i32 = 0x1104;
            pub const SPECIAL_HI_ATTACK_X_DIRECTION: i32 = 0x1105;
            pub const SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME: i32 = 0x1106;
            pub const SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME: i32 = 0x1107;
        }
    }

    pub mod luigi {
        pub mod instance {
            // flag
            /// This flag stores whether or not Luigi currently has a misfire stored.
            pub const IS_MISFIRE_STORED: i32 = 0x0100;
            
            // int
            /// This int stores the number of remaining green missile's luigi must do before getting a misfire
            pub const REMAINING_SPECIAL_S_UNTIL_MISFIRE: i32 = 0x0100;
            /// This int stores the handle of the charge smoke effect for killing it if we store misfire
            pub const CHARGE_SMOKE_EFFECT_HANDLE: i32 = 0x0101;
            /// This int stores the handle of the pulsing effect for killing it if we store misfire
            pub const CHARGE_PULSE_EFFECT_HANDLE: i32 = 0x0102;

            // float
            /// This float holds the current multiplier on damage for misfire
            pub const MISFIRE_DAMAGE_MULTIPLIER: i32 = 0x0100;
        }
        pub mod status {
            // flag
            pub use super::super::mario::status::IS_SPECIAL_N_FIREBRAND;
            pub use super::super::mario::status::IS_SPECIAL_N_DOUBLE_FIREBALL;
        }
    }

    pub mod mario {
        pub mod instance {
            // flags
            pub const NOKNOK_SHELL:                          i32 = 0x0100;
            pub const CAN_INPUT_SPECIAL_N_DOUBLE_FIREBALL:   i32 = 0x0101;
            pub const SPECIAL_N_DOUBLE_FIREBALL_NOTIFY_FLAG: i32 = 0x0102;
        }

        pub mod status {
            // flags
            pub const AERIAL_COMMAND_MOMENTUM_RESET: i32 = 0x1100;
            pub const AERIAL_COMMAND_RISING:         i32 = 0x1101;
            pub const AERIAL_COMMAND_RISEN:          i32 = 0x1102;

            pub const IS_SPECIAL_N_FIREBRAND:       i32 = 0x1100;
            pub const IS_SPECIAL_N_DOUBLE_FIREBALL: i32 = 0x1101;
        }
    }

    pub mod yoshi {
        pub mod status {
            pub use super::super::mario::status::{
                AERIAL_COMMAND_MOMENTUM_RESET,
                AERIAL_COMMAND_RISING,
                AERIAL_COMMAND_RISEN
            };
        }
    }

    pub mod master {
        pub mod status {
            // flags
            pub const IS_ENABLE_SPECIAL_S_DASH_CANCEL: i32 = 0x1100;
            pub const AIR_SPECIAL_S_AUTOCANCEL:        i32 = 0x1101;

            // ints
            pub const AYMR_CHARGE_LEVEL: i32 = 0x1100;
        }
    }

    pub mod mewtwo {
        pub mod instance {
            // flags
            pub const GROUNDED_TELEPORT: i32 = 0x0100;
            pub const UP_SPECIAL_JUMP_REFRESH: i32 = 0x0101;
        }
    }

    pub mod pickel {
        pub mod instance {
            // flags
            pub const IS_CURRENT_ATTACK_LW3_SOUL_FIRE: i32 = 0x0100;
            pub const TUMBLE_START: i32 = 0x0101;
            pub const IS_IN_TUMBLE: i32 = 0x0102;
        }
        pub mod status {
            // floats
            pub const GLIDE_TIMER: i32 = 0x1100;
        }
    }

    pub mod pikachu {
        pub mod instance {
            pub const DISABLE_QA_JC: i32 = 0x0100;
        }
    }

    pub mod pzenigame {
        pub mod instance {
            pub const WITHDRAW_FRAME: i32 = 0x0100;
        }
    }

    pub mod mariod {
        pub mod status {
            // flags
            pub const IS_SPECIAL_N_CHILL_PILL:       i32 = 0x1100;

            pub const IS_SPECIAL_S_ELECTRIC_BLANKET: i32 = 0x1100;

            pub const IS_SPECIAL_HI_UNABLE_CANCEL:   i32 = 0x1100;
        }
    }

    pub mod robin {
        pub mod status {
            // flags
            pub const ELWIND1_CANCEL: i32 = 0x1100;
        }
    }

    pub mod roy {
        pub mod instance {
            // flags
            pub const TRAIL_EFFECT: i32 = 0x0100;
        }
    }

    pub mod shotos {
        pub mod instance {
            // flags
            pub const IS_USE_EX_SPECIAL:      i32 = 0x0100;
            pub const IS_MAGIC_SERIES_CANCEL: i32 = 0x0101;
            pub const IS_ENABLE_FADC:         i32 = 0x0102;
            pub const IS_TARGET_COMBO_1:      i32 = 0x0103;
            pub const IS_TARGET_COMBO_2:      i32 = 0x0104;
            pub const IS_CURRENT_HADOKEN_EX:  i32 = 0x0105;

            // ints
            pub const REPEAT_COUNT_LW:      i32 = 0x0100;
            pub const REPEAT_COUNT_HI:      i32 = 0x0101;
            pub const EX_SPECIAL_SCRIPTING: i32 = 0x0102;
            pub const AIR_CHAIN_COMBO_NUM:  i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SHOULD_COMBOS_SCALE:           i32 = 0x1100;
            pub const REPEAT_INCREMENTED:            i32 = 0x1101;
            pub const IS_ENABLE_MAGIC_SERIES_CANCEL: i32 = 0x1102;
        }
    }

    pub mod shizue {
        pub mod instance {
            // floats
            pub const STORED_BALLOON_POWER: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const IS_NOT_QUICK_RELEASE: i32 = 0x1100;
        }
    }

    pub mod simon {
        pub mod status {
            pub const AIR_CROSS: i32 = 0x1100;
        }
    }

    pub mod sonic {
        pub mod status {
            // flags
            pub const PULSE_HITBOX: i32 = 0x1100;
        }
    }
    
    pub mod snake {
        pub mod instance {
            // ints
            pub const SNAKE_GRENADE_COUNTER: i32 = 0x0100;
        }
    }
    
    pub mod trail {
        pub mod instance {
            // flags
            pub const ATTACK_12_INTO_S3: i32 = 0x0100;
            pub const COMBO_PLUS_GROUND: i32 = 0x0101;
            pub const COMBO_PLUS_AIR: i32 = 0x0102;

        }
        pub mod status {
            // flags
            pub const SHOULD_PRORATE_ATTACK: i32 = 0x1100;

            pub const IS_LAND_CANCEL_THUNDER: i32 = 0x1100;
            pub const IS_GRAND_MAGIC: i32 = 0x1101;
            pub const IS_CURRENT_FIRAGA_GRAND_MAGIC: i32 = 0x1102;
            pub const IS_CURRENT_BLIZZAGA_GRAND_MAGIC: i32 = 0x1103;
            pub const IS_CURRENT_THUNDAGA_GRAND_MAGIC: i32 = 0x1104;

            pub const SIDE_SPECIAL_HIT: i32 = 0x1100;
            pub const IS_SIDE_SPECIAL_INPUT: i32 = 0x1101;
            pub const STOP_SIDE_SPECIAL: i32 = 0x1102;
            pub const UP_SPECIAL_TO_SIDE_SPECIAL: i32 = 0x1103;

            pub const UP_SPECIAL_HIT: i32 = 0x1100;

            // floats
            pub const SONIC_BLADE_Y: i32 = 0x1100;

        }
    }

    pub mod samus {
        pub mod instance {
            // flags
            pub const SHINESPARK_USED: i32 = 0x0100;
            pub const SHINESPARK_READY: i32 = 0x0101;
        }
    }

    pub mod robot {
        pub mod instance {
            // ints
            pub const PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE: i32 = 0x0100;
            pub const PREV_FUEL_THRESHOLD:                  i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const BOOST_ATTACK: i32 = 0x1100;
        }
    }

    pub mod palutena {
        pub mod status {
            // flags
            pub const SPECIAL_LW_AEGIS_REFLECTOR: i32 = 0x1100;

            // floats
            pub const SPECIAL_LW_LR: i32 = 0x1100;
        }
        pub mod instance {
            // flags
            pub const GROUNDED_TELEPORT: i32 = 0x0100;
            pub const UP_SPECIAL_JUMP_REFRESH: i32 = 0x0101;
        }
    }

    pub mod miiswordsman {
        pub mod instance {
            // flags
            pub const CHAKRAM_STICK_ATTACK: i32 = 0x0100;
            pub const SKYWARD_SLASH_DASH_HIT: i32 = 0x0101;

            // ints
            pub const SPECIAL_LW1_CHARGE_LEVEL: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const WAVE_SPECIAL_N: i32 = 0x1100;

            pub const GALE_STAB_EDGE_CANCEL: i32 = 0x1100;

            pub const SPECIAL_LW1_ATTACK_TRIGGER: i32 = 0x1100;
        }
    }

    pub mod miigunner {
        pub mod status {
            // floats
            pub const CHARGE_ATTACK_LEVEL: i32 = 0x1100;
            pub const IS_CHARGE_FINISHED: i32 = 0x1101;
        }
    }

    pub mod metaknight {
        pub mod instance {
            // flags
            pub const COMPLETED_SET_SPEEDS: i32 = 0x0100;
            pub const META_QUICK_NEED_SET_SPEEDS: i32 = 0x0101;
            pub const META_QUICK_PLAY_VC: i32 = 0x0102;
            pub const NEUTRAL_SPECIAL_HIT: i32 = 0x0103;
            pub const SIDE_SPECIAL_HIT: i32 = 0x0104;
            pub const UP_SPECIAL_HIT: i32 = 0x0105;
            pub const DOWN_SPECIAL_HIT: i32 = 0x0106;

            // ints
            /// i32 timer for color flashing when meta quick is ready
            pub const META_QUICK_READY_FLASH_TIMER: i32 = 0x0100;
            pub const META_QUICK_STATUS: i32 = 0x0101;

            /// u32 effect handle
            pub const META_QUICK_EFFECT_HANDLE: i32 = 0x0102;
            pub const META_QUICK_EFFECT_HANDLE2: i32 = 0x0103;

            /// u32 effect handle for the charge, 0xFFFF_FFFF for invalid
            pub const META_QUICK_CHARGE_EFFECT_HANDLE: i32 = 0x0104;
        }
    }
    
    // pub mod miifighter {
    //     // floats
    //     pub const CHARGE_ATTACK_LEVEL: i32 = 0x1000;
        
    //     // flags
    //     pub const IS_COUNTER_THROW_PARRIED_ATTACK: i32 = 0x1000;
    //     pub const IS_CURRENT_IRONBALL_HEAVY:       i32 = 0x1001;

    // }
    
    pub mod littlemac {
        pub mod status {
            // flags
            pub const IS_DREAMLAND_EXPRESS: i32 = 0x1100;
            pub const IS_LATE_DLE_INPUT: i32 = 0x1101;
        }
    }

    pub mod pichu {
        pub mod instance {
            //flags
            pub const IS_CHARGE_ATTACK: i32 = 0x0102;

            //ints
            pub const CHARGE_LEVEL: i32 = 0x0100;
            pub const CHARGE_EFFECT_HANDLER: i32 = 0x0101;

            // floats
            pub const CHARGE_DAMAGE_MUL: i32 = 0x0100;
            pub const CHARGE_RECOIL_MUL: i32 = 0x0101;
        }
    }

}

pub mod statuses {
    pub mod elight {
        pub const SPECIAL_HI_FINISH2: i32 = 0;
    }

    pub mod metaknight {
        pub const METAQUICK_SUMMON: i32 = 0;
    }

    pub mod ganon {
        pub const SPECIAL_N_FLOAT: i32 = 0;
    }

    pub mod ryu {
        pub const AIR_DASH: i32 = 0;
    }

}