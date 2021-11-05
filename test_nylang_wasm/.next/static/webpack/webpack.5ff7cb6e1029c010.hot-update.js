"use strict";
/*
 * ATTENTION: An "eval-source-map" devtool has been used.
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file with attached SourceMaps in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
self["webpackHotUpdate_N_E"]("webpack",{},
/******/ function(__webpack_require__) { // webpackRuntimeModules
/******/ /* webpack/runtime/async module */
/******/ !function() {
/******/ 	var webpackThen = typeof Symbol === "function" ? Symbol("webpack then") : "__webpack_then__";
/******/ 	var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 	var completeQueue = function(queue) {
/******/ 		if(queue) {
/******/ 			queue.forEach(function(fn) { fn.r--; });
/******/ 			queue.forEach(function(fn) { fn.r-- ? fn.r++ : fn(); });
/******/ 		}
/******/ 	}
/******/ 	var completeFunction = function(fn) { !--fn.r && fn(); };
/******/ 	var queueFunction = function(queue, fn) { queue ? queue.push(fn) : completeFunction(fn); };
/******/ 	var wrapDeps = function(deps) { return deps.map(function(dep) {
/******/ 		if(dep !== null && typeof dep === "object") {
/******/ 			if(dep[webpackThen]) return dep;
/******/ 			if(dep.then) {
/******/ 				var queue = [];
/******/ 				dep.then(function(r) {
/******/ 					obj[webpackExports] = r;
/******/ 					completeQueue(queue);
/******/ 					queue = 0;
/******/ 				});
/******/ 				var obj = {};
/******/ 											obj[webpackThen] = function(fn, reject) { queueFunction(queue, fn), dep.catch(reject); };
/******/ 				return obj;
/******/ 			}
/******/ 		}
/******/ 		var ret = {};
/******/ 							ret[webpackThen] = function(fn) { completeFunction(fn); };
/******/ 							ret[webpackExports] = dep;
/******/ 							return ret;
/******/ 	}); };
/******/ 	__webpack_require__.a = function(module, body, hasAwait) {
/******/ 		var queue = hasAwait && [];
/******/ 		var exports = module.exports;
/******/ 		var currentDeps;
/******/ 		var outerResolve;
/******/ 		var reject;
/******/ 		var isEvaluating = true;
/******/ 		var nested = false;
/******/ 		var whenAll = function(deps, onResolve, onReject) {
/******/ 			if (nested) return;
/******/ 			nested = true;
/******/ 			onResolve.r += deps.length;
/******/ 			deps.map(function(dep, i) { dep[webpackThen](onResolve, onReject); });
/******/ 			nested = false;
/******/ 		};
/******/ 		var promise = new Promise(function(resolve, rej) {
/******/ 			reject = rej;
/******/ 			outerResolve = function() { resolve(exports), completeQueue(queue), queue = 0; };
/******/ 		});
/******/ 		promise[webpackExports] = exports;
/******/ 		promise[webpackThen] = function(fn, rejectFn) {
/******/ 			if (isEvaluating) { return completeFunction(fn); }
/******/ 			if (currentDeps) whenAll(currentDeps, fn, rejectFn);
/******/ 			queueFunction(queue, fn);
/******/ 			promise.catch(rejectFn);
/******/ 		};
/******/ 		module.exports = promise;
/******/ 		body(function(deps) {
/******/ 			if(!deps) return outerResolve();
/******/ 			currentDeps = wrapDeps(deps);
/******/ 			var fn, result;
/******/ 			var promise = new Promise(function(resolve, reject) {
/******/ 				fn = function() { resolve(result = currentDeps.map(function(d) { return d[webpackExports]; })); };
/******/ 				fn.r = 0;
/******/ 				whenAll(currentDeps, fn, reject);
/******/ 			});
/******/ 			return fn.r ? promise : result;
/******/ 		}).then(outerResolve, reject);
/******/ 		isEvaluating = false;
/******/ 	};
/******/ }();
/******/ 
/******/ /* webpack/runtime/getFullHash */
/******/ !function() {
/******/ 	__webpack_require__.h = function() { return "9db27589b59d290c"; }
/******/ }();
/******/ 
/******/ /* webpack/runtime/wasm loading */
/******/ !function() {
/******/ 	__webpack_require__.v = function(exports, wasmModuleId, wasmModuleHash, importsObj) {
/******/ 		var req = fetch(__webpack_require__.p + "static/wasm/" + wasmModuleHash + ".wasm");
/******/ 		if (typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 			return WebAssembly.instantiateStreaming(req, importsObj)
/******/ 				.then(function(res) { return Object.assign(exports, res.instance.exports); });
/******/ 		}
/******/ 		return req
/******/ 			.then(function(x) { return x.arrayBuffer(); })
/******/ 			.then(function(bytes) { return WebAssembly.instantiate(bytes, importsObj); })
/******/ 			.then(function(res) { return Object.assign(exports, res.instance.exports); });
/******/ 	};
/******/ }();
/******/ 
/******/ /* webpack/runtime/compat */
/******/ 
/******/ 
/******/ // noop fns to prevent runtime errors during initialization
/******/ if (typeof self !== "undefined") {
/******/ 	self.$RefreshReg$ = function () {};
/******/ 	self.$RefreshSig$ = function () {
/******/ 		return function (type) {
/******/ 			return type;
/******/ 		};
/******/ 	};
/******/ }
/******/ 
/******/ }
);