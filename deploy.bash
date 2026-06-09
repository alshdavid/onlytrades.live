set -e

HOST="192.248.157.139"
USER="root"
REMOTE="$USER@$HOST"

cargo build --release 

echo Shutting down
ssh $REMOTE systemctl stop ctrader || true
ssh $REMOTE systemctl status ctrader || true
ssh $REMOTE rm /root/ctrader || true

echo Copying
scp ./target/release/onlytrades_trader $REMOTE:/root/ctrader

echo Starting
ssh $REMOTE systemctl start ctrader
ssh $REMOTE systemctl status ctrader
