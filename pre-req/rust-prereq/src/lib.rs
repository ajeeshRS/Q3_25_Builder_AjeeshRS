#[cfg(test)]
mod tests {
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        blake3::hash,
        instruction::{AccountMeta, Instruction},
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        system_program,
        transaction::Transaction,
    };
    use std::str::FromStr;
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated new solana wallet : {}",
            kp.pubkey().to_string()
        );
        println!("");

        println!("To save your wallet copy and paste the following into a JSON file");

        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your Base58-encoded private key is:");

        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str =
            "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

        // const RPC_URL: &str = "https://api.devnet.solana.com";

        fn claim_airdrop() {
            let keypair = read_keypair_file("dev-wallet.json").expect("could not find JSON file");

            let client = RpcClient::new(RPC_URL);

            let balance = client
                .get_balance(&keypair.pubkey())
                .expect("could not get the balance");
            println!("balance : {}", balance);
            
            // match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            //     Ok(sig) => {
            //         println!("Success! check your tx here");
            //         println!("{}", sig);
            //     }
            //     Err(err) => {
            //         println!("Airdrop failed : {}", err);
            //     }
            // }
        }

        claim_airdrop();
    }

    #[test]
    fn transfer_sol() {
        let keypair =
            read_keypair_file("dev-wallet.json").expect("could not find dev wallet json file");

        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";

        let sig = keypair.sign_message(message_bytes);

        let sig_hashed = hash(sig.as_ref());

        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("verification failed"),
        }

        let to_pubkey = Pubkey::from_str("EF5ZDRqSGfbTpiXQ8R7Mj4xdFehpSeVFPiYa9Fpnwg4E").unwrap();

        const RPC_URL: &str =
            "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get the recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let tx = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .expect("failed to send final transaction");

        println!("Success here is your tx signature : {}", sig);
    }

    #[test]
    fn submit() {
        const RPC_URL: &str =
            "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

        let rpc_client = RpcClient::new(RPC_URL);

        let signer =
            read_keypair_file("Turbin3-wallet.json").expect("could not find dev wallet json file");

        let mint = Keypair::new();
        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();

        let system_program = system_program::id();

        let signer_pubkey = signer.pubkey();

        let seeds = &[b"prereqs", signer_pubkey.as_ref()];

        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

        let authority_seed = &[b"collection", collection.as_ref()];
        let (authority, _bump) =
            Pubkey::find_program_address(authority_seed, &turbin3_prereq_program);

        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false),     // PDA account
            AccountMeta::new(mint.pubkey(), true),   // mint keypair
            AccountMeta::new(collection, false),     // collection
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false),
        ]; // system program];

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get latest blockhash");

        let ix = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer_pubkey),
            &[&signer, &mint],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send the tx");

        println!("success submit rs : {}", signature);
    }
}
