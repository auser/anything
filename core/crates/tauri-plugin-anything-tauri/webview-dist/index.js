function e(e,t,n,r){return new(n||(n=Promise))((function(o,i){function u(e){try{c(r.next(e))}catch(e){i(e)}}function a(e){try{c(r.throw(e))}catch(e){i(e)}}function c(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(u,a)}c((r=r.apply(e,t||[])).next())}))}function t(e,t){var n,r,o,i,u={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:a(0),throw:a(1),return:a(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function a(a){return function(c){return function(a){if(n)throw new TypeError("Generator is already executing.");for(;i&&(i=0,a[0]&&(u=0)),u;)try{if(n=1,r&&(o=2&a[0]?r.return:a[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,a[1])).done)return o;switch(r=0,o&&(a=[2&a[0],o.value]),a[0]){case 0:case 1:o=a;break;case 4:return u.label++,{value:a[1],done:!1};case 5:u.label++,r=a[1],a=[0];continue;case 7:a=u.ops.pop(),u.trys.pop();continue;default:if(!(o=u.trys,(o=o.length>0&&o[o.length-1])||6!==a[0]&&2!==a[0])){u=0;continue}if(3===a[0]&&(!o||a[1]>o[0]&&a[1]<o[3])){u.label=a[1];break}if(6===a[0]&&u.label<o[1]){u.label=o[1],o=a;break}if(o&&u.label<o[2]){u.label=o[2],u.ops.push(a);break}o[2]&&u.ops.pop(),u.trys.pop();continue}a=t.call(e,u)}catch(e){a=[6,e],r=0}finally{n=o=0}if(5&a[0])throw a[1];return{value:a[0]?a[1]:void 0,done:!0}}([a,c])}}}"function"==typeof SuppressedError&&SuppressedError;var n=Object.defineProperty;function r(e,t=!1){let n=window.crypto.getRandomValues(new Uint32Array(1))[0],r=`_${n}`;return Object.defineProperty(window,r,{value:n=>(t&&Reflect.deleteProperty(window,r),e?.(n)),writable:!1,configurable:!0}),n}async function o(e,t={}){return new Promise(((n,o)=>{let i=r((e=>{n(e),Reflect.deleteProperty(window,`_${u}`)}),!0),u=r((e=>{o(e),Reflect.deleteProperty(window,`_${i}`)}),!0);window.__TAURI_IPC__({cmd:e,callback:i,error:u,...t})}))}function i(e,t="asset"){return window.__TAURI__.convertFileSrc(e,t)}((e,t)=>{for(var r in t)n(e,r,{get:t[r],enumerable:!0})})({},{convertFileSrc:()=>i,invoke:()=>o,transformCallback:()=>r});var u=function(){function n(e){this.path=e}return n.prototype.stop=function(){return e(this,void 0,void 0,(function(){return t(this,(function(e){switch(e.label){case 0:return[4,o("plugin:anything-tauri|stop",{})];case 1:return[2,e.sent()]}}))}))},n.prototype.getFlows=function(){return e(this,void 0,void 0,(function(){return t(this,(function(e){switch(e.label){case 0:return[4,o("plugin:anything|get_flows",{path:this.path})];case 1:return[2,e.sent()]}}))}))},n.prototype.createFlow=function(n,r){return e(this,void 0,void 0,(function(){return t(this,(function(e){switch(e.label){case 0:return[4,o("plugin:anything|create_flow",{path:this.path,flowName:n,flowId:r})];case 1:return[2,e.sent()]}}))}))},n}();export{u as Anything};
