declare -a arr=(
    "ABDKMath64x64"
    "DarkForestArtifactUtils"
    "DarkForestCore"
    "DarkForestGPTCredit"
    "DarkForestGetters"
    "DarkForestInitialize"
    "DarkForestLazyUpdate"
    "DarkForestPlanet"
    "DarkForestStorageV1"
    "DarkForestTokens"
    "DarkForestTypes"
    "DarkForestUtils"
    "Verifier"
    "Whitelist"
)

for abi in "${arr[@]}" 
do
    cat $abi.sol/$abi.json | jq -r '.abi' > $abi.abi
done
