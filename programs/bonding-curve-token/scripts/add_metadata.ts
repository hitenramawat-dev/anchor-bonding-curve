import * as dotenv from 'dotenv';
dotenv.config();
import { AnchorProvider, BN, Idl, Program, setProvider, web3 } from "@coral-xyz/anchor";
import idl from "../../../target/idl/bonding_curve_token.json"
import { ComputeBudgetProgram, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { min } from 'bn.js';
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';



const main = async () => { 
    const provider = AnchorProvider.env()
    setProvider(provider);

    const program = new Program(idl as Idl,provider);

    const signer = provider.wallet;
   
    const mint = new PublicKey("6TGiU1EYmfSkCZVdQGBPj8tempxXQdj9WzgzVe1S6XpD")
    const METADATA_PROGRAM_ID = new PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

        console.log("Signer:", signer.publicKey.toString());
        console.log("Mint:", mint.toString());
   
    const [pda_metadata,bump_metadata] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"),METADATA_PROGRAM_ID.toBuffer(),mint.toBuffer()],
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
        mintAccount:mint,
        metadataAccount:pda_metadata,
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
