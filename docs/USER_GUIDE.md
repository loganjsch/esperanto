## Intro

This user guide uses policy.rs instead of the /policies. (which is coming, just not yet)

Also ensure you have access to the nitro enclave instance (for attesting AWS) and the code with my script that gens the docker and signs it and all that jazz. And also the split python scripts for request att and sending on both ec2 and enclave.

There are two EC2 instances being used - 

Esperanto EC2 which holds esperanto project and runs the esperanto-server which listens on port [don't remember]

Nitro Enclave instance which is the taret compute environment and sends the bytes to port [don't remember]



## Highlevel Steps (Work in Progress)

1. ON EC2 W/ NITRO: Start and sign nitro enclave via the [SCRIPT_NAME] script.

2. Record PCR Measurements (simulates POST /enroll)

3. ON ESPERANTO EC2: Copy Measurements into lib.rs (simulate policy resolution)

4. ON ESPERANTO EC2: Start esperanto-server - ./esperanto-server after having run cargo build i think

5. ON EC2 W/ NITRO: run parent python script (this tells the enclave via vsock to request an attestation from the nsm_api and then sends bytes back over vsock to parent which sends to)

6. Esperanto-server collects the bytes and saves to a file in target somewhere. (would need to modufy handler to send straight to verifier.rs)

7. Copy that doc file into ("tests/fixtures/payload_dump.bin")

8. Now cargo test and we're happy shoud pass all 3 tests.




