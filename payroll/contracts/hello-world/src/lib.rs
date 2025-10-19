// contracts/payroll/src/lib.rs

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol};

// --- Định nghĩa các khóa để lưu trữ dữ liệu ---
// Dùng Symbol để tiết kiệm chi phí
const ADMIN: Symbol = symbol_short!("ADMIN");
const SALARIES: Symbol = symbol_short!("SALARIES");

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    /// Hàm khởi tạo, được gọi một lần duy nhất khi triển khai contract.
    /// Hàm này sẽ thiết lập địa chỉ ví của người triển khai làm admin.
    pub fn initialize(env: Env, admin: Address) {
        // Kiểm tra xem admin đã được khởi tạo chưa, để tránh bị gọi lại
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract has already been initialized");
        }
        // Lưu địa chỉ admin vào storage
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// [Chỉ Admin] Dùng để thiết lập hoặc cập nhật lương cho một nhân viên.
    pub fn set_salary(env: Env, employee: Address, amount: u128) {
        // Lấy địa chỉ admin đã lưu
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        // Yêu cầu chữ ký của admin. Nếu người gọi không phải admin, lệnh sẽ thất bại.
        admin.require_auth();

        // Lấy danh sách lương từ storage, hoặc tạo mới nếu chưa có
        let mut salaries: Map<Address, u128> = env
            .storage()
            .instance()
            .get(&SALARIES)
            .unwrap_or_else(|| Map::new(&env));

        // Thiết lập lương cho địa chỉ ví của nhân viên
        salaries.set(employee, amount);

        // Lưu lại danh sách lương đã cập nhật vào storage
        env.storage().instance().set(&SALARIES, &salaries);
    }

    /// [Bất kỳ ai] Dùng để xem mức lương của một nhân viên.
    pub fn get_salary(env: Env, employee: Address) -> u128 {
        // Lấy danh sách lương
        let salaries: Map<Address, u128> = env
            .storage()
            .instance()
            .get(&SALARIES)
            .unwrap_or_else(|| Map::new(&env));

        // Trả về mức lương của nhân viên, nếu không tìm thấy thì trả về 0
        salaries.get(employee).unwrap_or(0)
    }
}