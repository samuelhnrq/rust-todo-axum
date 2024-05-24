"use strict";
/*
 * Copyright 2017 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
 * in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing permissions and
 * limitations under the License.
 */
(function(){function e(t,r,o){function n(s,a){if(!r[s]){if(!t[s]){var u="function"==typeof require&&require;if(!a&&u)return u(s,!0);if(i)return i(s,!0);var c=Error("Cannot find module '"+s+"'");throw c.code="MODULE_NOT_FOUND",c}var h=r[s]={exports:{}};t[s][0].call(h.exports,function(e){return n(t[s][1][e]||e)},h,h.exports,e,t,r,o)}return r[s].exports}for(var i="function"==typeof require&&require,s=0;s<o.length;s++)n(o[s]);return n}return e})()({1:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.App=void 0;var o=e("../authorization_request"),n=e("../authorization_request_handler"),i=e("../authorization_service_configuration"),s=e("../logger"),a=e("../redirect_based_handler"),u=e("../token_request"),c=e("../token_request_handler"),h="511828570984-7nmej36h9j2tebiqmpqh835naet4vci4.apps.googleusercontent.com",p="http://localhost:8000/app/redirect.html",l=function(){function e(e){var t=this;this.snackbar=e,this.notifier=new n.AuthorizationNotifier,this.authorizationHandler=new a.RedirectRequestHandler,this.tokenHandler=new c.BaseTokenRequestHandler,this.authorizationHandler.setAuthorizationNotifier(this.notifier),this.notifier.setAuthorizationListener(function(e,r,o){(0,s.log)("Authorization request complete ",e,r,o),r&&(t.request=e,t.response=r,t.code=r.code,t.showMessage("Authorization Code ".concat(r.code)))})}return e.prototype.showMessage=function(e){this.snackbar.MaterialSnackbar.showSnackbar({message:e})},e.prototype.fetchServiceConfiguration=function(){var e=this;i.AuthorizationServiceConfiguration.fetchFromIssuer("https://accounts.google.com").then(function(t){(0,s.log)("Fetched service configuration",t),e.configuration=t,e.showMessage("Completed fetching configuration")}).catch(function(t){(0,s.log)("Something bad happened",t),e.showMessage("Something bad happened ".concat(t))})},e.prototype.makeAuthorizationRequest=function(){var e=new o.AuthorizationRequest({client_id:h,redirect_uri:p,scope:"openid",response_type:o.AuthorizationRequest.RESPONSE_TYPE_CODE,state:void 0,extras:{prompt:"consent",access_type:"offline"}});this.configuration?this.authorizationHandler.performAuthorizationRequest(this.configuration,e):this.showMessage("Fetch Authorization Service configuration, before you make the authorization request.")},e.prototype.makeTokenRequest=function(){var e=this;if(!this.configuration){this.showMessage("Please fetch service configuration.");return}var t=null;if(this.code){var r=void 0;this.request&&this.request.internal&&((r={}).code_verifier=this.request.internal.code_verifier),t=new u.TokenRequest({client_id:h,redirect_uri:p,grant_type:u.GRANT_TYPE_AUTHORIZATION_CODE,code:this.code,refresh_token:void 0,extras:r})}else this.tokenResponse&&(t=new u.TokenRequest({client_id:h,redirect_uri:p,grant_type:u.GRANT_TYPE_REFRESH_TOKEN,code:void 0,refresh_token:this.tokenResponse.refreshToken,extras:void 0}));t&&this.tokenHandler.performTokenRequest(this.configuration,t).then(function(t){var r=!1;e.tokenResponse?(e.tokenResponse.accessToken=t.accessToken,e.tokenResponse.issuedAt=t.issuedAt,e.tokenResponse.expiresIn=t.expiresIn,e.tokenResponse.tokenType=t.tokenType,e.tokenResponse.scope=t.scope):(r=!0,e.tokenResponse=t),e.code=void 0,r?e.showMessage("Obtained a refresh token ".concat(t.refreshToken)):e.showMessage("Obtained an access token ".concat(t.accessToken,"."))}).catch(function(t){(0,s.log)("Something bad happened",t),e.showMessage("Something bad happened ".concat(t))})},e.prototype.checkForAuthorizationResponse=function(){this.authorizationHandler.completeAuthorizationRequestIfPossible()},e}();r.App=l,window.App=l},{"../authorization_request":2,"../authorization_request_handler":3,"../authorization_service_configuration":5,"../logger":9,"../redirect_based_handler":11,"../token_request":13,"../token_request_handler":14}],2:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.AuthorizationRequest=void 0;var o=e("./crypto_utils"),n=e("./logger"),i=function(){function e(t,r,n){var i;void 0===r&&(r=new o.DefaultCrypto),void 0===n&&(n=!0),this.crypto=r,this.usePkce=n,this.clientId=t.client_id,this.redirectUri=t.redirect_uri,this.scope=t.scope,this.responseType=t.response_type||e.RESPONSE_TYPE_CODE,this.state=t.state||(i=r).generateRandom(10),this.extras=t.extras,this.internal=t.internal}return e.prototype.setupCodeVerifier=function(){var e=this;if(!this.usePkce)return Promise.resolve();var t=this.crypto.generateRandom(128);return this.crypto.deriveChallenge(t).catch(function(e){(0,n.log)("Unable to generate PKCE challenge. Not using PKCE",e)}).then(function(r){r&&(e.internal=e.internal||{},e.internal.code_verifier=t,e.extras=e.extras||{},e.extras.code_challenge=r,e.extras.code_challenge_method="S256")})},e.prototype.toJson=function(){var e=this;return this.setupCodeVerifier().then(function(){return{response_type:e.responseType,client_id:e.clientId,redirect_uri:e.redirectUri,scope:e.scope,state:e.state,extras:e.extras,internal:e.internal}})},e.RESPONSE_TYPE_TOKEN="token",e.RESPONSE_TYPE_CODE="code",e}();r.AuthorizationRequest=i},{"./crypto_utils":6,"./logger":9}],3:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.AuthorizationRequestHandler=r.BUILT_IN_PARAMETERS=r.AuthorizationNotifier=void 0;var o=e("./logger"),n=e("./url_validator"),i=function(){function e(){this.listener=null}return e.prototype.setAuthorizationListener=function(e){this.listener=e},e.prototype.onAuthorizationComplete=function(e,t,r){this.listener&&this.listener(e,t,r)},e}();r.AuthorizationNotifier=i,r.BUILT_IN_PARAMETERS=["redirect_uri","client_id","response_type","state","scope"];var s=function(){function e(e,t){this.utils=e,this.crypto=t,this.notifier=null}return e.prototype.buildRequestUrl=function(e,t){var o={redirect_uri:t.redirectUri,client_id:t.clientId,response_type:t.responseType,state:t.state,scope:t.scope};if(t.extras)for(var i in t.extras)t.extras.hasOwnProperty(i)&&0>r.BUILT_IN_PARAMETERS.indexOf(i)&&(o[i]=t.extras[i]);var s=this.utils.stringify(o),a=(0,n.requireValidUrl)(e.authorizationEndpoint);return"".concat(a,"?").concat(s)},e.prototype.completeAuthorizationRequestIfPossible=function(){var e=this;return(0,o.log)("Checking to see if there is an authorization response to be delivered."),this.notifier||(0,o.log)("Notifier is not present on AuthorizationRequest handler.\n          No delivery of result will be possible"),this.completeAuthorizationRequest().then(function(t){t||(0,o.log)("No result is available yet."),t&&e.notifier&&e.notifier.onAuthorizationComplete(t.request,t.response,t.error)})},e.prototype.setAuthorizationNotifier=function(e){return this.notifier=e,this},e}();r.AuthorizationRequestHandler=s},{"./logger":9,"./url_validator":16}],4:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.AuthorizationError=r.AuthorizationResponse=void 0;var o=function(){function e(e){this.code=e.code,this.state=e.state}return e.prototype.toJson=function(){return{code:this.code,state:this.state}},e}();r.AuthorizationResponse=o;var n=function(){function e(e){this.error=e.error,this.errorDescription=e.error_description,this.errorUri=e.error_uri,this.state=e.state}return e.prototype.toJson=function(){return{error:this.error,error_description:this.errorDescription,error_uri:this.errorUri,state:this.state}},e}();r.AuthorizationError=n},{}],5:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.AuthorizationServiceConfiguration=void 0;var o=e("./xhr"),n=function(){function e(e){this.authorizationEndpoint=e.authorization_endpoint,this.tokenEndpoint=e.token_endpoint,this.revocationEndpoint=e.revocation_endpoint,this.userInfoEndpoint=e.userinfo_endpoint,this.endSessionEndpoint=e.end_session_endpoint}return e.prototype.toJson=function(){return{authorization_endpoint:this.authorizationEndpoint,token_endpoint:this.tokenEndpoint,revocation_endpoint:this.revocationEndpoint,end_session_endpoint:this.endSessionEndpoint,userinfo_endpoint:this.userInfoEndpoint}},e.fetchFromIssuer=function(t,r){var n="".concat(t,"/").concat(".well-known","/").concat("openid-configuration");return(r||new o.JQueryRequestor).xhr({url:n,dataType:"json",method:"GET"}).then(function(t){return new e(t)})},e}();r.AuthorizationServiceConfiguration=n},{"./xhr":17}],6:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.DefaultCrypto=r.textEncodeLite=r.urlSafe=r.bufferToString=void 0;var o=e("base64-js"),n=e("./errors"),i="undefined"!=typeof window&&!!window.crypto,s=i&&!!window.crypto.subtle,a="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";function u(e){for(var t=[],r=0;r<e.byteLength;r+=1){var o=e[r]%a.length;t.push(a[o])}return t.join("")}function c(e){return o.fromByteArray(new Uint8Array(e)).replace(/\+/g,"-").replace(/\//g,"_").replace(/=/g,"")}function h(e){for(var t=new ArrayBuffer(e.length),r=new Uint8Array(t),o=0;o<e.length;o++)r[o]=e.charCodeAt(o);return r}r.bufferToString=u,r.urlSafe=c,r.textEncodeLite=h;var p=function(){function e(){}return e.prototype.generateRandom=function(e){var t=new Uint8Array(e);if(i)window.crypto.getRandomValues(t);else for(var r=0;r<e;r+=1)t[r]=Math.random()*a.length|0;return u(t)},e.prototype.deriveChallenge=function(e){return e.length<43||e.length>128?Promise.reject(new n.AppAuthError("Invalid code length.")):s?new Promise(function(t,r){crypto.subtle.digest("SHA-256",h(e)).then(function(e){return t(c(new Uint8Array(e)))},function(e){return r(e)})}):Promise.reject(new n.AppAuthError("window.crypto.subtle is unavailable."))},e}();r.DefaultCrypto=p},{"./errors":7,"base64-js":18}],7:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.AppAuthError=void 0,r.AppAuthError=function e(t,r){this.message=t,this.extras=r}},{}],8:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.IS_PROFILE=r.IS_LOG=r.setFlag=r.Flags=void 0,r.Flags={IS_LOG:!0,IS_PROFILE:!1},r.setFlag=function e(t,o){r.Flags[t]=o},r.IS_LOG=r.Flags.IS_LOG,r.IS_PROFILE=r.Flags.IS_PROFILE},{}],9:[function(e,t,r){"use strict";var o=this&&this.__spreadArray||function(e,t,r){if(r||2===arguments.length)for(var o,n=0,i=t.length;n<i;n++)!o&&n in t||(o||(o=Array.prototype.slice.call(t,0,n)),o[n]=t[n]);return e.concat(o||Array.prototype.slice.call(t))};Object.defineProperty(r,"__esModule",{value:!0}),r.profile=r.log=void 0;var n=e("./flags");function i(e){for(var t=[],r=1;r<arguments.length;r++)t[r-1]=arguments[r];n.Flags.IS_LOG&&((t?t.length:0)>0?console.log.apply(console,o([e],t,!1)):console.log(e))}r.log=i;var s="undefined"!=typeof window&&!!window.performance&&!!console.profile;r.profile=function e(t,r,a){var u,c,h,p,l;return n.Flags.IS_PROFILE?(u=t,c=r,h=a,p=h.value,l=p.name,l||(l="anonymous function"),s?h.value=function(e){console.profile(l);var t=window.performance.now(),r=p.call.apply(p,o([this||window],e,!1)),n=window.performance.now()-t;return console.log("".concat(l," took ").concat(n," ms")),console.profileEnd(),r}:h.value=function(e){i("Profile start ".concat(l));var t=Date.now(),r=p.call.apply(p,o([this||window],e,!1)),n=Date.now()-t;return i("Profile end ".concat(l," took ").concat(n," ms.")),r},h):a}},{"./flags":8}],10:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.BasicQueryStringUtils=void 0;var o=function(){function e(){}return e.prototype.parse=function(e,t){return t?this.parseQueryString(e.hash):this.parseQueryString(e.search)},e.prototype.parseQueryString=function(e){for(var t={},r=(e=e.trim().replace(/^(\?|#|&)/,"")).split("&"),o=0;o<r.length;o+=1){var n=r[o].split("=");if(n.length>=2){var i=decodeURIComponent(n.shift()),s=n.length>0?n.join("="):null;s&&(t[i]=decodeURIComponent(s))}}return t},e.prototype.stringify=function(e){var t=[];for(var r in e)e.hasOwnProperty(r)&&e[r]&&t.push("".concat(encodeURIComponent(r),"=").concat(encodeURIComponent(e[r])));return t.join("&")},e}();r.BasicQueryStringUtils=o},{}],11:[function(e,t,r){"use strict";var o,n=this&&this.__extends||(o=function(e,t){return(o=Object.setPrototypeOf||({__proto__:[]})instanceof Array&&function(e,t){e.__proto__=t}||function(e,t){for(var r in t)Object.prototype.hasOwnProperty.call(t,r)&&(e[r]=t[r])})(e,t)},function(e,t){if("function"!=typeof t&&null!==t)throw TypeError("Class extends value "+String(t)+" is not a constructor or null");function r(){this.constructor=e}o(e,t),e.prototype=null===t?Object.create(t):(r.prototype=t.prototype,new r)});Object.defineProperty(r,"__esModule",{value:!0}),r.RedirectRequestHandler=void 0;var i=e("./authorization_request"),s=e("./authorization_request_handler"),a=e("./authorization_response"),u=e("./crypto_utils"),c=e("./logger"),h=e("./query_string_utils"),p=e("./storage"),l=function(e){return"".concat(e,"_appauth_authorization_request")},f=function(e){return"".concat(e,"_appauth_authorization_service_configuration")},d="appauth_current_authorization_request",v=function(e){function t(t,r,o,n){void 0===t&&(t=new p.LocalStorageBackend),void 0===r&&(r=new h.BasicQueryStringUtils),void 0===o&&(o=window.location),void 0===n&&(n=new u.DefaultCrypto);var i=e.call(this,r,n)||this;return i.storageBackend=t,i.locationLike=o,i}return n(t,e),t.prototype.performAuthorizationRequest=function(e,t){var r=this,o=this.crypto.generateRandom(10);Promise.all([this.storageBackend.setItem(d,o),t.toJson().then(function(e){return r.storageBackend.setItem(l(o),JSON.stringify(e))}),this.storageBackend.setItem(f(o),JSON.stringify(e.toJson())),]).then(function(){var o=r.buildRequestUrl(e,t);(0,c.log)("Making a request to ",t,o),r.locationLike.assign(o)})},t.prototype.completeAuthorizationRequest=function(){var e=this;return this.storageBackend.getItem(d).then(function(t){return t?e.storageBackend.getItem(l(t)).then(function(e){return JSON.parse(e)}).then(function(e){return new i.AuthorizationRequest(e)}).then(function(r){var o="".concat(e.locationLike.origin).concat(e.locationLike.pathname),n=e.utils.parse(e.locationLike,!0),i=n.state,s=n.code,u=n.error;(0,c.log)("Potential authorization request ",o,n,i,s,u);var h=i===r.state,p=null,v=null;if(!h)return(0,c.log)("Mismatched request (state and request_uri) dont match."),Promise.resolve(null);if(u){var g=n.error_uri,y=n.error_description;v=new a.AuthorizationError({error:u,error_description:y,error_uri:g,state:i})}else p=new a.AuthorizationResponse({code:s,state:i});return Promise.all([e.storageBackend.removeItem(d),e.storageBackend.removeItem(l(t)),e.storageBackend.removeItem(f(t))]).then(function(){return(0,c.log)("Delivering authorization response"),{request:r,response:p,error:v}})}):null})},t}(s.AuthorizationRequestHandler);r.RedirectRequestHandler=v},{"./authorization_request":2,"./authorization_request_handler":3,"./authorization_response":4,"./crypto_utils":6,"./logger":9,"./query_string_utils":10,"./storage":12}],12:[function(e,t,r){"use strict";var o,n=this&&this.__extends||(o=function(e,t){return(o=Object.setPrototypeOf||({__proto__:[]})instanceof Array&&function(e,t){e.__proto__=t}||function(e,t){for(var r in t)Object.prototype.hasOwnProperty.call(t,r)&&(e[r]=t[r])})(e,t)},function(e,t){if("function"!=typeof t&&null!==t)throw TypeError("Class extends value "+String(t)+" is not a constructor or null");function r(){this.constructor=e}o(e,t),e.prototype=null===t?Object.create(t):(r.prototype=t.prototype,new r)});Object.defineProperty(r,"__esModule",{value:!0}),r.LocalStorageBackend=r.StorageBackend=void 0;var i=function e(){};r.StorageBackend=i;var s=function(e){function t(t){var r=e.call(this)||this;return r.storage=t||window.localStorage,r}return n(t,e),t.prototype.getItem=function(e){var t=this;return new Promise(function(r,o){var n=t.storage.getItem(e);r(n||null)})},t.prototype.removeItem=function(e){var t=this;return new Promise(function(r,o){t.storage.removeItem(e),r()})},t.prototype.clear=function(){var e=this;return new Promise(function(t,r){e.storage.clear(),t()})},t.prototype.setItem=function(e,t){var r=this;return new Promise(function(o,n){r.storage.setItem(e,t),o()})},t}(i);r.LocalStorageBackend=s},{}],13:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.TokenRequest=r.GRANT_TYPE_REFRESH_TOKEN=r.GRANT_TYPE_AUTHORIZATION_CODE=void 0,r.GRANT_TYPE_AUTHORIZATION_CODE="authorization_code",r.GRANT_TYPE_REFRESH_TOKEN="refresh_token";var o=function(){function e(e){this.clientId=e.client_id,this.redirectUri=e.redirect_uri,this.grantType=e.grant_type,this.code=e.code,this.refreshToken=e.refresh_token,this.extras=e.extras}return e.prototype.toJson=function(){return{grant_type:this.grantType,code:this.code,refresh_token:this.refreshToken,redirect_uri:this.redirectUri,client_id:this.clientId,extras:this.extras}},e.prototype.toStringMap=function(){var e={grant_type:this.grantType,client_id:this.clientId,redirect_uri:this.redirectUri};if(this.code&&(e.code=this.code),this.refreshToken&&(e.refresh_token=this.refreshToken),this.extras)for(var t in this.extras)this.extras.hasOwnProperty(t)&&!e.hasOwnProperty(t)&&(e[t]=this.extras[t]);return e},e}();r.TokenRequest=o},{}],14:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.BaseTokenRequestHandler=void 0;var o=e("./errors"),n=e("./query_string_utils"),i=e("./token_response"),s=e("./xhr"),a=function(){function e(e,t){void 0===e&&(e=new s.JQueryRequestor),void 0===t&&(t=new n.BasicQueryStringUtils),this.requestor=e,this.utils=t}return e.prototype.isTokenResponse=function(e){return void 0===e.error},e.prototype.performRevokeTokenRequest=function(e,t){return this.requestor.xhr({url:e.revocationEndpoint,method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded"},data:this.utils.stringify(t.toStringMap())}).then(function(e){return!0})},e.prototype.performTokenRequest=function(e,t){var r=this;return this.requestor.xhr({url:e.tokenEndpoint,method:"POST",dataType:"json",headers:{"Content-Type":"application/x-www-form-urlencoded"},data:this.utils.stringify(t.toStringMap())}).then(function(e){return r.isTokenResponse(e)?new i.TokenResponse(e):Promise.reject(new o.AppAuthError(e.error,new i.TokenError(e)))})},e}();r.BaseTokenRequestHandler=a},{"./errors":7,"./query_string_utils":10,"./token_response":15,"./xhr":17}],15:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.TokenError=r.TokenResponse=r.nowInSeconds=void 0;var o=function(){return Math.round(new Date().getTime()/1e3)};r.nowInSeconds=o;var n=function(){function e(e){this.accessToken=e.access_token,this.tokenType=e.token_type||"bearer",e.expires_in&&(this.expiresIn=parseInt(e.expires_in,10)),this.refreshToken=e.refresh_token,this.scope=e.scope,this.idToken=e.id_token,this.issuedAt=e.issued_at||(0,r.nowInSeconds)()}return e.prototype.toJson=function(){var e;return{access_token:this.accessToken,id_token:this.idToken,refresh_token:this.refreshToken,scope:this.scope,token_type:this.tokenType,issued_at:this.issuedAt,expires_in:null===(e=this.expiresIn)||void 0===e?void 0:e.toString()}},e.prototype.isValid=function(e){return void 0===e&&(e=-600),!this.expiresIn||(0,r.nowInSeconds)()<this.issuedAt+this.expiresIn+e},e}();r.TokenResponse=n;var i=function(){function e(e){this.error=e.error,this.errorDescription=e.error_description,this.errorUri=e.error_uri}return e.prototype.toJson=function(){return{error:this.error,error_description:this.errorDescription,error_uri:this.errorUri}},e}();r.TokenError=i},{}],16:[function(e,t,r){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),r.requireValidUrl=void 0;var o=e("./errors");r.requireValidUrl=function e(t){try{return new URL(t),t}catch(r){throw new o.AppAuthError("Invalid input url ".concat(t))}}},{"./errors":7}],17:[function(e,t,r){"use strict";var o,n=this&&this.__extends||(o=function(e,t){return(o=Object.setPrototypeOf||({__proto__:[]})instanceof Array&&function(e,t){e.__proto__=t}||function(e,t){for(var r in t)Object.prototype.hasOwnProperty.call(t,r)&&(e[r]=t[r])})(e,t)},function(e,t){if("function"!=typeof t&&null!==t)throw TypeError("Class extends value "+String(t)+" is not a constructor or null");function r(){this.constructor=e}o(e,t),e.prototype=null===t?Object.create(t):(r.prototype=t.prototype,new r)});Object.defineProperty(r,"__esModule",{value:!0}),r.TestRequestor=r.FetchRequestor=r.JQueryRequestor=r.Requestor=void 0;var i=e("./errors"),s=function e(){};r.Requestor=s;var a=function(e){function t(){return null!==e&&e.apply(this,arguments)||this}return n(t,e),t.prototype.xhr=function(e){var t=$.ajax(e);return new Promise(function(e,r){t.then(function(t,r,o){e(t)},function(e,t,o){r(new i.AppAuthError(o))})})},t}(s);r.JQueryRequestor=a;var u=function(e){function t(){return null!==e&&e.apply(this,arguments)||this}return n(t,e),t.prototype.xhr=function(e){if(!e.url)return Promise.reject(new i.AppAuthError("A URL must be provided."));var t=new URL(e.url),r={};if(r.method=e.method,r.mode="cors",e.data&&(e.method&&"POST"===e.method.toUpperCase()?r.body=e.data:new URLSearchParams(e.data).forEach(function(e,r){t.searchParams.append(r,e)})),r.headers={},e.headers)for(var o in e.headers)e.headers.hasOwnProperty(o)&&(r.headers[o]=e.headers[o]);var n=e.dataType&&"json"===e.dataType.toLowerCase();return n&&(r.headers.Accept="application/json, text/javascript, */*; q=0.01"),fetch(t.toString(),r).then(function(e){if(!(e.status>=200)||!(e.status<300))return Promise.reject(new i.AppAuthError(e.status.toString(),e.statusText));var t=e.headers.get("content-type");return n||t&&-1!==t.indexOf("application/json")?e.json():e.text()})},t}(s);r.FetchRequestor=u;var c=function(e){function t(t){var r=e.call(this)||this;return r.promise=t,r}return n(t,e),t.prototype.xhr=function(e){return this.promise},t}(s);r.TestRequestor=c},{"./errors":7}],18:[function(e,t,r){"use strict";r.byteLength=function e(t){var r=c(t),o=r[0],n=r[1];return(o+n)*3/4-n},r.toByteArray=function e(t){var r,o,s,a,u,h=c(t),p=h[0],l=h[1],f=new i((a=p,u=l,(a+u)*3/4-u)),d=0,v=l>0?p-4:p;for(o=0;o<v;o+=4)r=n[t.charCodeAt(o)]<<18|n[t.charCodeAt(o+1)]<<12|n[t.charCodeAt(o+2)]<<6|n[t.charCodeAt(o+3)],f[d++]=r>>16&255,f[d++]=r>>8&255,f[d++]=255&r;return 2===l&&(r=n[t.charCodeAt(o)]<<2|n[t.charCodeAt(o+1)]>>4,f[d++]=255&r),1===l&&(r=n[t.charCodeAt(o)]<<10|n[t.charCodeAt(o+1)]<<4|n[t.charCodeAt(o+2)]>>2,f[d++]=r>>8&255,f[d++]=255&r),f},r.fromByteArray=function e(t){for(var r,n=t.length,i=n%3,s=[],a=0,u=n-i;a<u;a+=16383)s.push(p(t,a,a+16383>u?u:a+16383));return 1===i?s.push(o[(r=t[n-1])>>2]+o[r<<4&63]+"=="):2===i&&s.push(o[(r=(t[n-2]<<8)+t[n-1])>>10]+o[r>>4&63]+o[r<<2&63]+"="),s.join("")};for(var o=[],n=[],i="undefined"!=typeof Uint8Array?Uint8Array:Array,s="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",a=0,u=s.length;a<u;++a)o[a]=s[a],n[s.charCodeAt(a)]=a;function c(e){var t=e.length;if(t%4>0)throw Error("Invalid string. Length must be a multiple of 4");var r=e.indexOf("=");-1===r&&(r=t);var o=r===t?0:4-r%4;return[r,o]}function h(e){return o[e>>18&63]+o[e>>12&63]+o[e>>6&63]+o[63&e]}function p(e,t,r){for(var o,n=[],i=t;i<r;i+=3)n.push(h(o=(e[i]<<16&16711680)+(e[i+1]<<8&65280)+(255&e[i+2])));return n.join("")}n["-".charCodeAt(0)]=62,n["_".charCodeAt(0)]=63},{}]},{},[1]);