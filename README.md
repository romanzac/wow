# Wow

Word of Wisdom TCP server/client example implementation with Argon2d as a Proof of Work algorithm.

#### To clone the repository, use the following commands:

```sh
git clone https://github.com/romanzac/wow
```

#### Build docker images
```sh
cd wow
./scripts/build_docker.sh 
```

#### Run Wow Server:
```sh
./scripts/start_wserver_docker.sh
```

#### Run Wow Client:
```sh
./scripts/start_wclient_docker.sh
```
Client will connect to server and request challenge, provide hash as a PoW and receive quote. 



