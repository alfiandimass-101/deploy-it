// ===========================================
// BEVY ECS CHEATSHEET (Ringkasan Cepat)
// ===========================================

// Bevy ECS adalah arsitektur Entity Component System (ECS) utama di Bevy.
// Tujuannya: Sederhana, cepat, dan sangat paralel.

// --- 1. Konsep Inti ECS ---

// ENTITY: ID unik (seperti "barang" di dunia game) yang TIDAK menyimpan data.
//         Data disimpan di Components.
// COMPONENTS: Struct Rust biasa yang menyimpan DATA untuk Entities.
//             Contoh: struct Position { x: f32, y: f32 }
// SYSTEMS: Fungsi Rust biasa yang berisi LOGIKA/kode pemrosesan.
//          Berjalan pada Entities yang memiliki Component yang sesuai (melalui Query).

// WORLD: Tempat penyimpanan semua Entities, Components, dan Resources.

// RESOURCES: Data singleton global di World. Tidak terikat pada Entity mana pun.
//            Contoh: Waktu, Pengaturan Volume, Renderer.
//            Diakses di System menggunakan 'Res<T>' atau 'ResMut<T>'.

// --- 2. Cara Kerja Dasar ---

// QUERY: Cara System untuk meminta data (Components) dari sekelompok Entities.
//        Hanya memproses Entities yang cocok dengan daftar Components yang diminta.
//        fn system_nama(query: Query<(&mut ComponentA, &ComponentB)>) { ... }

// SCHEDULES: Mendefinisikan urutan/strategi eksekusi Systems.
//            Bevy menjalankan sebanyak mungkin Systems secara PARALEL.

// --- 3. Fitur Utama ---

// BUNDLES: Gabungan beberapa Components dalam satu Struct.
//          Memudahkan saat 'world.spawn(Bundle::default())'.

// CHANGE DETECTION: Melacak perubahan Components/Resources.
//                   Query bisa difilter untuk mendeteksi:
//                   - 'Changed<C>': Component C telah dimutasi sejak System terakhir berjalan.
//                   - 'Added<C>': Component C baru ditambahkan ke Entity.

// QUERY FILTERS: Memperketat hasil Query:
//                - '(With<C>, Without<D>)': Hanya Entity dengan C TAPI tanpa D.

// OBSERVERS / TRIGGERS: Sistem 'reaktif' yang berjalan segera.
//                       Mereka merespons ketika Event (Pemicu) terjadi.
//                       Berbeda dengan Systems biasa yang berjalan di Schedule.
//                       world.add_observer(|event: On<EventKu>| { ... });

// COMPONENT STORAGE: Cara data Component disimpan di belakang layar.
//                    - Tables (Default): Baik untuk iterasi cepat (cache friendly).
//                    - Sparse Sets: Lebih cepat saat menambah/menghapus Component.

use azalea::{app::{App, Plugin, Startup, Update}, prelude::*};
use crate::bot::component::BotComponent;

/// Plugin untuk logika inti bot.
pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_message)
            .add_systems(Update, tick_counter);
    }
}

/// Sistem yang berjalan sekali saat startup untuk menampilkan pesan.
fn startup_message() {
    println!("CoreLogicPlugin berhasil dimuat!");
}

fn tick_counter(mut query: Query<&mut BotComponent>) {
    for mut bot in query.iter_mut() {
        bot.tick += 1;
        if bot.tick > 20 {
            bot.tick = 0;
        }
        println!("Tick: {}", bot.tick);
    }
}
