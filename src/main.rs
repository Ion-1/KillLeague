#![windows_subsystem = "windows"]

use qmetaobject::prelude::*;
use std::ffi::OsStr;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[derive(QObject, Default)]
struct Backend {
    base: qt_base_class!(trait QObject),

    kill_league: qt_method!(
        fn kill_league(&mut self) {
            self.kill_proc(OsStr::new("League of Legends.exe"));
        }
    ),

    kill_client: qt_method!(
        fn kill_client(&mut self) {
            self.kill_proc(OsStr::new("LeagueClient.exe"));
        }
    ),

    kill_riot: qt_method!(
        fn kill_riot(&mut self) {
            self.kill_proc(OsStr::new("Riot Client.exe"));
        }
    ),

    kill_service: qt_method!(
        fn kill_service(&mut self) {
            self.kill_proc(OsStr::new("RiotClientServices.exe"));
        }
    ),

    pub system: System,
}
impl Backend {
    fn kill_proc(&mut self, name: &OsStr) {
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );
        self.system
            .processes_by_exact_name(name)
            .for_each(|proc| {proc.kill();});
    }
}

fn main() {
    let mut engine = QmlEngine::new();

    let backend = QObjectBox::new(Backend::default());
    engine.set_object_property(QString::from("Backend"), backend.pinned());

    engine.load_data(include_str!("ui.qml").into());

    engine.exec()
}
