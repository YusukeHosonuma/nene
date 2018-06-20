lib_name="nene"

if [ -z "$1" ]; then
    echo "Please specify tag."
    exit 1
fi

tag=$1

# build
cargo build --release

# archive
zip -j nene.zip ./target/release/nene
sha256=$(shasum -a 256 nene.zip | cut -d ' ' -f 1)

# update formula
formula_path="$lib_name.rb"
formula_url="https://api.github.com/repos/YusukeHosonuma/homebrew-$lib_name/contents/$formula_path"
sha=`curl GET $formula_url | jq -r '.sha'`
content_encoded=`cat formula.rb.tmpl | sed -e "s/{{TAG}}/$tag/" | sed -e "s/{{SHA256}}/$sha256/" | openssl enc -e -base64 | tr -d '\n '`

commit_message="Update version to $tag"

curl -i -X PUT $formula_url \
   -H "Content-Type:application/json" \
   -H "Authorization:token $GITHUB_TOKEN" \
   -d \
"{
 \"path\":\"$formula_path\",
 \"sha\":\"$sha\",
 \"content\":\"$content_encoded\",
 \"message\":\"$commit_message\"
}"

# git tag
git tag $tag
git push origin $tag

# Github release
github-release release \
    --user YusukeHosonuma \
    --repo nene \
    --tag $tag

# Github upload
github-release upload \
    --user YusukeHosonuma \
    --repo nene \
    --tag $tag \
    --name $lib_name.zip \
    --file $lib_name.zip

# clean
rm $lib_name.zip
