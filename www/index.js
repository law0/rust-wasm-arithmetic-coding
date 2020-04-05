import { WasmCompress } from 'wasm-compress'

const wasmCompress = new WasmCompress()

const myFunction = () => {
  var inputText = document.getElementById('theString').value
  wasmCompress.set_string(inputText)
  document.getElementById('orig').innerHTML = 'Original string : ' + wasmCompress.get_string()
  document.getElementById('result').innerHTML = 'result string : ' + wasmCompress.get_result_string()
  document.getElementById('f').innerHTML = 'last compressed float :' + wasmCompress.get_result_float()

  document.getElementById('ratio').innerHTML = 'compression ratio : ' + wasmCompress.get_compression_ratio()

  document.getElementById('tree').innerHTML = wasmCompress.get_tree_string()
}

document.getElementById('button1').addEventListener('click', myFunction)
