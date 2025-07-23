import * as dotenv from 'dotenv';
dotenv.config();
import { AnchorProvider, BN, Idl, Program, setProvider, web3 } from "@coral-xyz/anchor";
import idl from "../../../target/idl/bonding_curve_token.json"
import { ComputeBudgetProgram, PublicKey, SystemProgram } from '@solana/web3.js';
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token';



const main = async () => {
    const provider = AnchorProvider.env()
    setProvider(provider);

    const program = new Program(idl as Idl,provider);

    const signer = provider.wallet;
   
    const mint = new PublicKey("CCkAdGCk4qyabgoUe5qDv7vHtQeE53cxPe96fyzfMjCv")

        console.log("Signer:", signer.publicKey.toString());
        console.log("Mint:", mint.toString());
   
    //b"", signer.key().as_ref(), mint_account.key().as_ref()
    const [pda_bonding,bump_bonding] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("bonding_curve"),signer.publicKey.toBuffer(),mint.toBuffer()],
        program.programId
    )

    const signer_ata = getAssociatedTokenAddressSync(mint,signer.publicKey,false);

    //b"vault_for_sol",bonding_pda.key().as_ref()
    const [pda_vault,bump_vault] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault_for_sol"),pda_bonding.toBuffer()],
        program.programId
    )

    //b"fee_vault", bonding_pda.key().as_ref()
    const [pda_fee,bump_fee] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fee_vault"),pda_bonding.toBuffer()],
        program.programId
    )

    console.log("Bonding PDA:", pda_bonding.toString());
    console.log("Vault PDA:", pda_vault.toString());
    console.log("Fee PDA:", pda_fee.toString());
    console.log("Signer ATA:", signer_ata.toString());

    let creator = new PublicKey("EpTZy8wg9YC6jerepiebigRXgBSGg3HNH6jBm4gbHj25");

    try {
        const tx  = await program.methods
        .sellTokens(creator,new BN(500),new BN(0))
        .accounts({
            signer:signer.publicKey,
            vault:pda_vault,
            mintAccount:mint,
            bondingPda:pda_bonding,
            ataAccountUser:signer_ata,
            feePda:pda_fee,
            systemProgram:web3.SystemProgram.programId,
            tokenProgram:TOKEN_PROGRAM_ID
        }).rpc()

        console.log("Transaction: ",tx);
        
    } catch (error) {
        console.log("Transaction Error:",error);
        
    }

}


main()