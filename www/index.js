import { WasmCompress } from 'wasm-compress'

const wasmCompress = new WasmCompress()

const myFunction = () => {
  var inputText = document.getElementById('theString').value
  wasmCompress.set_string(inputText)
  document.getElementById('demo').innerHTML = 'Original string : ' + wasmCompress.get_string() + '<br>'
  document.getElementById('demo').innerHTML += 'uncompressed string :' + wasmCompress.get_result_string() + '<br>'
  document.getElementById('demo').innerHTML += 'compressed float :' + wasmCompress.get_result_float()

  document.getElementById('tree').innerHTML = wasmCompress.get_tree_string()
}

document.getElementById('button1').addEventListener('click', myFunction)
