import {emit, listen} from '@tauri-apps/api/event'

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = await listen('click', event => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
})

// emits the `click` event with the object payload
emit('click', {
  theMessage: 'Tauri is awesome!'
})

import {appWindow, WebviewWindow} from '@tauri-apps/api/window'

// emit an event that are only visible to the current window
appWindow.emit('event', {message: 'Tauri is awesome!'})

// create a new webview window and emit an event only to that window
const webview = new WebviewWindow('window')
webview.emit('event')
