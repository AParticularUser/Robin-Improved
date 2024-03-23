pub mod param {
    pub const REFLET_INT_SWORD_CONSUME_POINT : i32 = 3;
    // pub const REFLET_INT_THUNDER_CONSUME_POINT : i32 = 1;
    pub const REFLET_FLOAT_FIRE_CONSUME_POINT : f32 = 2.0;
    // pub const REFLET_FLOAT_FIRE_ATTACK_13_CONSUME_POINT : f32 = 0.5;
    pub const REFLET_INT_WIND_CONSUME_POINT : i32 = 2;
    // pub const REFLET_INT_WIND_ATTACK_100_CONSUME_POINT : i32 = 1;
    pub const REFLET_INT_DARK_START_CONSUME_POINT : i32 = 6;
    pub const REFLET_INT_DARK_HOLD_CONSUME_POINT : i32 = 1;
    pub const REFLET_INT_DARK_CATCH_ATTACK_CONSUME_POINT : i32 = 1;

    pub const REFLET_INT_ATTACK_AIR_N_AB_SMASH_FRAME : i32 = 3;
    
    pub const REFLET_INT_SPECIAL_LW_HOLD_FRAME : i32 = 20;
    
    pub const REFLET_INT_SWORD_PASSIVE_RECHARGE_FRAME : i32 = 90;
    pub const REFLET_INT_THUNDER_PASSIVE_RECHARGE_FRAME : i32 = 90;
    pub const REFLET_INT_FIRE_PASSIVE_RECHARGE_FRAME : i32 = 90;
    pub const REFLET_INT_WIND_PASSIVE_RECHARGE_FRAME : i32 = 75;
    pub const REFLET_INT_DARK_PASSIVE_RECHARGE_FRAME : i32 = 97;
}
pub mod vars {
    pub mod instance {//0x0???
        //flag
        pub const REFLET_FLAG_SPECIAL_HI_2_ENABLE : i32 = 0x0000;
        //int
        pub const REFLET_INT_SWORD_PASSIVE_RECHARGE_COUNT : i32 = 0x0000;
        pub const REFLET_INT_THUNDER_PASSIVE_RECHARGE_COUNT : i32 = 0x0001;
        pub const REFLET_INT_FIRE_PASSIVE_RECHARGE_COUNT : i32 = 0x0002;
        pub const REFLET_INT_WIND_PASSIVE_RECHARGE_COUNT : i32 = 0x0003;
        pub const REFLET_INT_DARK_PASSIVE_RECHARGE_COUNT : i32 = 0x0004;
        //float
        // pub const REFLET_FLOAT_ : i32 = 0x0000;
    }
    pub mod status {//0x1???
        //flag
        pub const REFLET_FLAG_SPECIAL_HI_ENABLE_LANDING : i32 = 0x1000;
        pub const REFLET_FLAG_SPECIAL_LW_HOLD : i32 = 0x1000;
        //int
        pub const REFLET_INT_ATTACK_AIR_N_AB_SMASH_COUNT : i32 = 0x1000;
        pub const REFLET_INT_SPECIAL_LW_HOLD_COUNT : i32 = 0x1000;
        //float
        // pub const REFLET_FLOAT_ : i32 = 0x1000;
    }
}