import {defineConfig} from 'vite';
export default defineConfig({server:{host:'0.0.0.0',port:5173,allowedHosts:true,proxy:{'/api':{target:'http://127.0.0.1:3000',ws:true}}},build:{target:'es2022',sourcemap:true},preview:{host:'0.0.0.0',allowedHosts:true}})
