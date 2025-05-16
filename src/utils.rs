use smash::app;
use smash::lib::lua_const::*;
use smash::app::utility;

pub fn get_category(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    utility::get_category(boma) as i32
}

pub unsafe fn get_player_number(boma: &mut app::BattleObjectModuleAccessor) -> usize{
    app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}
