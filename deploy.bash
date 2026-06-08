set -e

cargo build --release 

echo Shutting down
ssh root@192.248.157.139 systemctl stop ctrader || true
ssh root@192.248.157.139 systemctl status ctrader || true
ssh root@192.248.157.139 rm /root/ctrader || true

echo Copying
scp ./target/release/onlytrades_trader root@192.248.157.139:/root/ctrader

echo Starting
ssh root@192.248.157.139 systemctl start ctrader
ssh root@192.248.157.139 systemctl status ctrader
