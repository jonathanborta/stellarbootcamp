#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec
};

// Storage key untuk data transaksi
const TOPUP_DATA: Symbol = symbol_short!("TP_DATA");

// Struktur data Topup
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopupOrder {
    pub id: u64,
    pub username: String,
    pub robux_amount: u32,
    pub price_idr: u64,
    pub is_completed: bool,
}

#[contract]
pub struct RobloxTopupContract;

#[contractimpl]
impl RobloxTopupContract {

    // ✅ Ambil semua riwayat topup (Read)
    pub fn get_all_orders(env: Env) -> Vec<TopupOrder> {
        env.storage()
            .instance()
            .get(&TOPUP_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ✅ Buat pesanan topup baru (Create)
    pub fn create_order(env: Env, username: String, robux_amount: u32, price_idr: u64) -> u64 {
        let mut orders: Vec<TopupOrder> = env.storage()
            .instance()
            .get(&TOPUP_DATA)
            .unwrap_or(Vec::new(&env));

        let order_id = env.prng().gen::<u64>();
        
        let new_order = TopupOrder {
            id: order_id,
            username,
            robux_amount,
            price_idr,
            is_completed: false, // Default pesanan baru belum selesai
        };

        orders.push_back(new_order);
        env.storage().instance().set(&TOPUP_DATA, &orders);

        order_id
    }

    // ✅ Update status topup menjadi selesai (Update)
    pub fn mark_as_completed(env: Env, id: u64) -> String {
        let orders: Vec<TopupOrder> = env.storage()
            .instance()
            .get(&TOPUP_DATA)
            .unwrap_or(Vec::new(&env));

        let mut updated_orders = Vec::new(&env);
        let mut found = false;

        for mut order in orders.iter() {
            if order.id == id {
                order.is_completed = true;
                found = true;
            }
            updated_orders.push_back(order);
        }

        if found {
            env.storage().instance().set(&TOPUP_DATA, &updated_orders);
            String::from_str(&env, "Status Topup diperbarui: Selesai")
        } else {
            String::from_str(&env, "Order ID tidak ditemukan")
        }
    }

    // ✅ Hapus riwayat topup (Delete)
    pub fn delete_order(env: Env, id: u64) -> String {
        let orders: Vec<TopupOrder> = env.storage()
            .instance()
            .get(&TOPUP_DATA)
            .unwrap_or(Vec::new(&env));

        let mut new_orders = Vec::new(&env);
        let mut found = false;

        for order in orders.iter() {
            if order.id != id {
                new_orders.push_back(order);
            } else {
                found = true;
            }
        }

        if found {
            env.storage().instance().set(&TOPUP_DATA, &new_orders);
            String::from_str(&env, "Data pesanan berhasil dihapus")
        } else {
            String::from_str(&env, "Data pesanan tidak ditemukan")
        }
    }
}

mod test;