# Creating a server
In case someone blows up the EC2, run these commands in a fresh install of Ubuntu
and then give Docker sometime to build the image. You should have a fresh server
after that.

```bash
sudo git clone https://github.com/PaoloMazzon/ByteBound.git/ /app/
cd /app
sudo chmod u+x docker/bootstrap.sh
sudo docker/bootstrap.sh
```