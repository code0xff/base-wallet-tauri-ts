<script>
  import Wallet from './Wallet'

  let mnemonic = ''
  let path = ''
  let hrp = ''

  let _private = '' 
  let _public = ''
  let ext_private = ''
  let ext_public = ''
  let _btc = ''
  let _eth = ''
  let _cosmos = ''
  let _evmos = ''
  let _message = ''

  async function generate(){
    mnemonic = await Wallet.generate()
  }

  async function derive(){
    const derived = await Wallet.derive(mnemonic, path, hrp)
    if (derived.success) {
      let [keypair, btc, eth, cosmos, evmos] = derived.result
      _private = keypair['private']
      _public = keypair['public']
      ext_private = keypair['ext_private']
      ext_public = keypair['ext_public']
      _btc = btc
      _eth = eth
      _cosmos = cosmos
      _evmos = evmos
      _message = ''
    } else {
      _message = derived.message
      _private = ''
      _public = ''
      ext_private = ''
      ext_public = ''
      _btc = ''
      _eth = ''
      _cosmos = ''
      _evmos = ''
    }
  }
</script>

<div class='component'>
  <input id='mnemonic' placeholder='mnemonic' bind:value={mnemonic} style='text-align: center;'/>
  <button on:click={generate}>
    GENERATE
  </button>
  <br/>
  <input id='path' placeholder='path (ex. m/44h/0h/0h/0/0)' bind:value={path} style='text-align: center;'/>
  <input id='hrp' placeholder='hrp (ex. cosmos, juno, osmo, etc.)' bind:value={hrp} style='text-align: center;'/>
  <button on:click={derive}>
    DERIVE
  </button>
  <p style='color: red;'>{_message}</p>
  <p>{_private}</p>
  <p>{_public}</p>
  <p>{ext_private}</p>
  <p>{ext_public}</p>
  <p>{_btc}</p>
  <p>{_eth}</p>
  <p>{_cosmos}</p>
  <p>{_evmos}</p>
</div>

<style>
</style>