const accounts = [
  {
    name: 'a',
    address: 'secret1jhwpq5mn7edw6lfv7sadae7g0qaq2atgpakf5j',
    mnemonic: 'auction finish supply wage icon praise prepare barrel recall bean van joke virtual connect only galaxy book twist post edit sport hamster build useful'
  },
  {
    name: 'b',
    address: 'secret17dgcylpm3wm6jawya9gzfv6hy9s7mnxhkkxrzf',
    mnemonic: 'crouch menu badge sunset check spawn plate loan book smooth off text skull lab snack decide guitar submit frost now daring equip screen weekend'
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
