import * as dotenv from 'dotenv';
dotenv.config();
import { AnchorProvider, BN, Idl, Program, setProvider, web3 } from "@coral-xyz/anchor";
import idl from "../../../target/idl/bonding_curve_token.json"
import { ComputeBudgetProgram, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { min } from 'bn.js';



const main = async () => {
    const provider = AnchorProvider.env()
    setProvider(provider);

    const program = new Program(idl as Idl,provider);

    const signer = provider.wallet;
    const mint = new Keypair();



        console.log("Signer:", signer.publicKey.toString());
        console.log("Mint:", mint.publicKey.toString());
   
    //b"", signer.key().as_ref(), mint_account.key().as_ref()
    const [pda_bonding,bump_bonding] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("bonding_curve"),signer.publicKey.toBuffer(),mint.publicKey.toBuffer()],
        program.programId
    )

    const signer_ata = getAssociatedTokenAddressSync(mint.publicKey,signer.publicKey,false);

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

    let tx = await program.methods.
    initializeCurve(new BN(100_00_00))
    .accounts({
    signer:signer.publicKey,
    bondingPda:pda_bonding,
    mintAccount:mint.publicKey,
    associatedTokenAccount:signer_ata,
    vault:pda_vault,
    feeVault:pda_fee,
    systemProgram:SystemProgram.programId,
    tokenProgram:TOKEN_PROGRAM_ID,
    associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([mint]).rpc()
    
    
    console.log("Tx:",tx);

    console.log("Transaction 1 Completed");
    

    const METADATA_PROGRAM_ID = new PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

        console.log("Signer:", signer.publicKey.toString());
        console.log("Mint:", mint.toString());
   
    const [pda_metadata,bump_metadata] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"),METADATA_PROGRAM_ID.toBuffer(),mint.publicKey.toBuffer()],
        METADATA_PROGRAM_ID
    )

    console.log("metadataAccount:",pda_metadata);
    let uri = "https://gist.githubusercontent.com/hitenramawat-dev/371f3baf01d40bbe4124631a3a1301fb/raw/6cba7ad7ec985430b3a5c2d3df7bf6d2538492eb/gistfile1.txt";

    const programAccount = await provider.connection.getAccountInfo(METADATA_PROGRAM_ID);
        if (!programAccount) {
            throw new Error("Metadata program account not found");
        }
        if (!programAccount.executable) {
            throw new Error("Metadata program account is not executable");
    }

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
        units: 300_000,
    });

   try {

    let tx = await program.methods
    .addMetadata("YEEZY","YZY",uri)
    .accounts({
        payer:signer.publicKey,
        mintAccount:mint.publicKey,
        metadataAccount:pda_metadata,
        bondingAccount:pda_bonding,
        tokenProgram:TOKEN_PROGRAM_ID,
        tokenMetadataProgram:METADATA_PROGRAM_ID,
        systemProgram:SystemProgram.programId,
        rent:SYSVAR_RENT_PUBKEY
    }).preInstructions([computeBudgetIx]).rpc()

    console.log("Transaction: ",tx);
   } catch (error) {
    console.log("Error:- ",error);
   }
}






main()


//signer
// bonding_pda
// mint_account
// ata_signer
// vault_pda
// fee_vault
// token
// system
// ata