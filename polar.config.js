const accounts = [
  {
    name: 'a',
    address: 'secret1ptuvyssd7rgspp0t6snsuxfy8txcpphl2m0map',
    mnemonic: 'comfort kick wing fever utility bamboo lamp electric champion urban inquiry ten card erode silly ugly river select wreck citizen dwarf insect surface club'
  },
  {
    name: 'b',
    address: 'secret1qvc4vald0jjh0wkhpjtt7vtrvhuq9fmxc5c9rc',
    mnemonic: 'replace antenna twice first half cook bounce manage mistake that wish jungle radar aim hover syrup exchange bone into want work veteran float amount'
  }
];

const networks = {
  default: {
    endpoint: "http://192.168.1.95:1337/"
  },
  localnet: {
    endpoint: 'http://192.168.1.95:1337/',
    accounts: accounts,
  },
  development: {
    endpoint: 'tcp://0.0.0.0:26656',
    chainId: 'secretdev-1',
    types: {}
  },
 
};

module.exports = {
  networks: {
    default: networks.localnet,
    localnet: networks.localnet,
    development: networks.development
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.55.0",
  }
};
