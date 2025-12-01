#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[multiversx_sc::contract]
pub trait CrowdfundingSc {
    
    // --- MODIFICACIÓ 1: INIT ARA REP ELS NOUS PARÀMETRES ---
    #[init]
    fn init(
        &self, 
        target: BigUint, 
        deadline: u64, 
        min_contribution: BigUint, // Mínim per transacció
        max_per_user: BigUint,     // Màxim acumulat per usuari
        max_cap: BigUint           // Màxim total del projecte (Hard Cap)
    ) {
        require!(target > 0, "Target must be more than 0");
        require!(max_cap >= target, "Max cap must be >= Target");
        require!(max_per_user >= min_contribution, "Max per user must be >= Min contribution");

        self.target().set(target);
        self.min_contribution().set(min_contribution);
        self.max_per_user().set(max_per_user);
        self.max_cap().set(max_cap);

        require!(
            deadline > self.get_current_time(),
            "Deadline can't be in the past"
        );
        self.deadline().set(deadline);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        let payment = self.call_value().egld().clone_value();

        // --- CONDICIÓ 1: Import mínim per transacció ---
        require!(
            payment >= self.min_contribution().get(),
            "L'import es inferior al minim permès per transaccio"
        );

        let current_time = self.blockchain().get_block_timestamp();
        require!(
            current_time < self.deadline().get(),
            "cannot fund after deadline"
        );

        // --- CONDICIÓ 2: Màxim total del projecte (Hard Cap) ---
        // get_current_funds inclou el pagament actual perquè és un mètode payable
        let current_funds = self.get_current_funds();
        require!(
            current_funds <= self.max_cap().get(),
            "S'ha superat el limit maxim de financament del projecte (Hard Cap)"
        );

        let caller = self.blockchain().get_caller();
        let deposited_amount = self.deposit(&caller).get();
        let new_user_total = &deposited_amount + &payment;

        // --- CONDICIÓ 3: Màxim acumulat per usuari ---
        require!(
            new_user_total <= self.max_per_user().get(),
            "Superes el limit maxim d'aportacio per usuari"
        );

        // Si tot és correcte, actualitzem el dipòsit de l'usuari
        self.deposit(&caller).set(new_user_total);
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance);
            }
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);
                }
            }
        }
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    // private

    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    // storage

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // --- MODIFICACIÓ 2: NOUS STORAGE MAPPERS ---
    
    #[view(getMinContribution)]
    #[storage_mapper("minContribution")]
    fn min_contribution(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxPerUser)]
    #[storage_mapper("maxPerUser")]
    fn max_per_user(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxCap)]
    #[storage_mapper("maxCap")]
    fn max_cap(&self) -> SingleValueMapper<BigUint>;
}