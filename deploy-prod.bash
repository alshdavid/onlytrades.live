set -e

cargo build --release 

echo Shutting down
ssh root@192.248.157.139 systemctl stop ctrader-prod || true
ssh root@192.248.157.139 systemctl status ctrader-prod || true
ssh root@192.248.157.139 rm /root/ctrader-prod || true

echo Copying
scp ./target/release/onlytrades_trader root@192.248.157.139:/root/ctrader-prod

echo Starting
ssh root@192.248.157.139 systemctl start ctrader-prod
ssh root@192.248.157.139 systemctl status ctrader-prod
