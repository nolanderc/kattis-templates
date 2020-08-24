cargo new $1
rm -rf $1/src
cp $(cd `dirname $0` && pwd)/*.rs $1
cp $(cd `dirname $0` && pwd)/Cargo.toml $1
