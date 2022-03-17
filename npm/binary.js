
const { Binary } = require("binary-install");
const os = require("os");
const cTable = require("console.table");

const error = msg => {
    console.error(msg);
    process.exit(1);
};

const { version, name, binaryname } = require("../package.json");

const repository = {
    url: "https://github.com/mgnlsol/ha-cli-wrapper"
}

const supportedPlatforms = [
    {
        TYPE: "Windows_NT",
        ARCHITECTURE: "x64",
        RUST_TARGET: "x86_64-pc-windows-msvc",
        ARCHIVE_NAME: "x86_64-pc-windows-gnu.tar.gz",
        BINARY_NAME: "ha-cli-wrapper.exe"
    },
    {
        TYPE: "Linux",
        ARCHITECTURE: "x64",
        RUST_TARGET: "x86_64-unknown-linux-musl",
        ARCHIVE_NAME: "x86_64-unknown-linux-musl.tar.gz",
        BINARY_NAME: "ha-cli-wrapper"
    },
    {
        TYPE: "Darwin",
        ARCHITECTURE: "x64",
        RUST_TARGET: "x86_64-apple-darwin",
        ARCHIVE_NAME: "x86_64-apple-darwin.tar.gz",
        BINARY_NAME: "ha-cli-wrapperd"
    }
];

const getPlatformMetadata = () => {
    const type = os.type();
    const architecture = os.arch();

    for (let index in supportedPlatforms) {
        let supportedPlatform = supportedPlatforms[index];
        if (
            type === supportedPlatform.TYPE &&
            architecture === supportedPlatform.ARCHITECTURE
        ) {
            return supportedPlatform;
        }
    }

    error(
        `Platform with type "${type}" and architecture "${architecture}" is not supported by ${binaryname}.\nYour system must be one of the following:\n\n${cTable.getTable(
            supportedPlatforms
        )}`
    );
};

const getBinary = () => {
    const platformMetadata = getPlatformMetadata();
    const url = `${repository.url}/releases/download/v${version}/${binaryname}_v${version}_${platformMetadata.ARCHIVE_NAME}`;    
    return new Binary(platformMetadata.BINARY_NAME, url);
};

const run = () => {
    const binary = getBinary();
    binary.run(); 
};

const install = () => {
    const binary = getBinary();
    binary.install();
};

module.exports = {
    install,
    run
};