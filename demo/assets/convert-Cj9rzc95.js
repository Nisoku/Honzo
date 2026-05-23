import { s as B, p as S, g as ot, b as st, f as at, e as ct, c as T, h as _t, _ as it } from "./honzo_wasm-B7nTe1NJ.js";
(async ()=>{
    const ut = "modulepreload", lt = function(e) {
        return "/Honzo/demo/" + e;
    }, $ = {}, ft = function(t, n, r) {
        let s = Promise.resolve();
        if (n && n.length > 0) {
            document.getElementsByTagName("link");
            const a = document.querySelector("meta[property=csp-nonce]"), c = a?.nonce || a?.getAttribute("nonce");
            s = Promise.allSettled(n.map((_)=>{
                if (_ = lt(_), _ in $) return;
                $[_] = !0;
                const f = _.endsWith(".css"), b = f ? '[rel="stylesheet"]' : "";
                if (document.querySelector(`link[href="${_}"]${b}`)) return;
                const d = document.createElement("link");
                if (d.rel = f ? "stylesheet" : ut, f || (d.as = "script"), d.crossOrigin = "", d.href = _, c && d.setAttribute("nonce", c), document.head.appendChild(d), f) return new Promise((k, i)=>{
                    d.addEventListener("load", k), d.addEventListener("error", ()=>i(new Error(`Unable to preload CSS for ${_}`)));
                });
            }));
        }
        function l(a) {
            const c = new Event("vite:preloadError", {
                cancelable: !0
            });
            if (c.payload = a, window.dispatchEvent(c), !c.defaultPrevented) throw a;
        }
        return s.then((a)=>{
            for (const c of a || [])c.status === "rejected" && l(c.reason);
            return t().catch(l);
        });
    };
    class P {
        __destroy_into_raw() {
            const t = this.__wbg_ptr;
            return this.__wbg_ptr = 0, q.unregister(this), t;
        }
        free() {
            const t = this.__destroy_into_raw();
            o.__wbg_wasmepubextractor_free(t, 0);
        }
        get_chapter(t) {
            return o.wasmepubextractor_get_chapter(this.__wbg_ptr, t);
        }
        get_chapter_count() {
            return o.wasmepubextractor_get_chapter_count(this.__wbg_ptr);
        }
        get_chapter_json(t) {
            return o.wasmepubextractor_get_chapter_json(this.__wbg_ptr, t);
        }
        get_chapter_resource(t, n) {
            const r = I(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = h;
            return o.wasmepubextractor_get_chapter_resource(this.__wbg_ptr, t, r, s);
        }
        get_chapter_text(t) {
            return o.wasmepubextractor_get_chapter_text(this.__wbg_ptr, t);
        }
        get_chapters_text() {
            return o.wasmepubextractor_get_chapters_text(this.__wbg_ptr);
        }
        get_chapters_text_json() {
            return o.wasmepubextractor_get_chapters_text_json(this.__wbg_ptr);
        }
        get_cover_image() {
            return o.wasmepubextractor_get_cover_image(this.__wbg_ptr);
        }
        get_cover_image_format() {
            return o.wasmepubextractor_get_cover_image_format(this.__wbg_ptr);
        }
        get_cover_image_len() {
            return o.wasmepubextractor_get_cover_image_len(this.__wbg_ptr);
        }
        get_metadata() {
            return o.wasmepubextractor_get_metadata(this.__wbg_ptr);
        }
        get_metadata_is_valid() {
            return o.wasmepubextractor_get_metadata_is_valid(this.__wbg_ptr);
        }
        get_metadata_json() {
            return o.wasmepubextractor_get_metadata_json(this.__wbg_ptr);
        }
        get_resource(t) {
            const n = I(t, o.__wbindgen_malloc, o.__wbindgen_realloc), r = h;
            return o.wasmepubextractor_get_resource(this.__wbg_ptr, n, r);
        }
        get_title() {
            return o.wasmepubextractor_get_title(this.__wbg_ptr);
        }
        get_toc() {
            return o.wasmepubextractor_get_toc(this.__wbg_ptr);
        }
        get_toc_json() {
            return o.wasmepubextractor_get_toc_json(this.__wbg_ptr);
        }
        get_total_char_count() {
            return o.wasmepubextractor_get_total_char_count(this.__wbg_ptr);
        }
        get_total_word_count() {
            return o.wasmepubextractor_get_total_word_count(this.__wbg_ptr);
        }
        has_cover() {
            return o.wasmepubextractor_has_cover(this.__wbg_ptr);
        }
        load_from_bytes(t) {
            return o.wasmepubextractor_load_from_bytes(this.__wbg_ptr, t);
        }
        constructor(){
            const t = o.wasmepubextractor_new();
            return this.__wbg_ptr = t, q.register(this, this.__wbg_ptr, this), this;
        }
        resolve_chapter_resource_path(t, n) {
            const r = I(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = h;
            return o.wasmepubextractor_resolve_chapter_resource_path(this.__wbg_ptr, t, r, s);
        }
    }
    Symbol.dispose && (P.prototype[Symbol.dispose] = P.prototype.free);
    function dt() {
        return {
            __proto__: null,
            "./lexepub_bg.js": {
                __proto__: null,
                __wbg_Error_3639a60ed15f87e7: function(t, n) {
                    return Error(C(t, n));
                },
                __wbg_String_8564e559799eccda: function(t, n) {
                    const r = String(n), s = I(r, o.__wbindgen_malloc, o.__wbindgen_realloc), l = h;
                    H().setInt32(t + 4, l, !0), H().setInt32(t + 0, s, !0);
                },
                __wbg___wbindgen_is_function_2f0fd7ceb86e64c5: function(t) {
                    return typeof t == "function";
                },
                __wbg___wbindgen_is_string_eddc07a3efad52e6: function(t) {
                    return typeof t == "string";
                },
                __wbg___wbindgen_is_undefined_244a92c34d3b6ec0: function(t) {
                    return t === void 0;
                },
                __wbg___wbindgen_throw_9c75d47bf9e7731e: function(t, n) {
                    throw new Error(C(t, n));
                },
                __wbg__wbg_cb_unref_158e43e869788cdc: function(t) {
                    t._wbg_cb_unref();
                },
                __wbg_call_a41d6421b30a32c5: function() {
                    return wt(function(t, n, r) {
                        return t.call(n, r);
                    }, arguments);
                },
                __wbg_length_ba3c032602efe310: function(t) {
                    return t.length;
                },
                __wbg_new_2fad8ca02fd00684: function() {
                    return new Object;
                },
                __wbg_new_3baa8d9866155c79: function() {
                    return new Array;
                },
                __wbg_new_46ae4e4ff2a07a64: function() {
                    return new Map;
                },
                __wbg_new_from_slice_5a173c243af2e823: function(t, n) {
                    return new Uint8Array(V(t, n));
                },
                __wbg_new_typed_1137602701dc87d4: function(t, n) {
                    try {
                        var r = {
                            a: t,
                            b: n
                        }, s = (a, c)=>{
                            const _ = r.a;
                            r.a = 0;
                            try {
                                return bt(_, r.b, a, c);
                            } finally{
                                r.a = _;
                            }
                        };
                        return new Promise(s);
                    } finally{
                        r.a = 0;
                    }
                },
                __wbg_prototypesetcall_fd4050e806e1d519: function(t, n, r) {
                    Uint8Array.prototype.set.call(V(t, n), r);
                },
                __wbg_queueMicrotask_40ac6ffc2848ba77: function(t) {
                    queueMicrotask(t);
                },
                __wbg_queueMicrotask_74d092439f6494c1: function(t) {
                    return t.queueMicrotask;
                },
                __wbg_resolve_9feb5d906ca62419: function(t) {
                    return Promise.resolve(t);
                },
                __wbg_set_6be42768c690e380: function(t, n, r) {
                    t[n] = r;
                },
                __wbg_set_82f7a370f604db70: function(t, n, r) {
                    return t.set(n, r);
                },
                __wbg_set_f614f6a0608d1d1d: function(t, n, r) {
                    t[n >>> 0] = r;
                },
                __wbg_static_accessor_GLOBAL_THIS_1c7f1bd6c6941fdb: function() {
                    const t = typeof globalThis > "u" ? null : globalThis;
                    return U(t) ? 0 : x(t);
                },
                __wbg_static_accessor_GLOBAL_e039bc914f83e74e: function() {
                    const t = typeof global > "u" ? null : global;
                    return U(t) ? 0 : x(t);
                },
                __wbg_static_accessor_SELF_8bf8c48c28420ad5: function() {
                    const t = typeof self > "u" ? null : self;
                    return U(t) ? 0 : x(t);
                },
                __wbg_static_accessor_WINDOW_6aeee9b51652ee0f: function() {
                    const t = typeof window > "u" ? null : window;
                    return U(t) ? 0 : x(t);
                },
                __wbg_then_20a157d939b514f5: function(t, n) {
                    return t.then(n);
                },
                __wbindgen_cast_0000000000000001: function(t, n) {
                    return pt(t, n, gt);
                },
                __wbindgen_cast_0000000000000002: function(t) {
                    return t;
                },
                __wbindgen_cast_0000000000000003: function(t, n) {
                    return C(t, n);
                },
                __wbindgen_cast_0000000000000004: function(t) {
                    return BigInt.asUintN(64, t);
                },
                __wbindgen_init_externref_table: function() {
                    const t = o.__wbindgen_externrefs, n = t.grow(4);
                    t.set(0, void 0), t.set(n + 0, void 0), t.set(n + 1, null), t.set(n + 2, !0), t.set(n + 3, !1);
                }
            }
        };
    }
    function gt(e, t, n) {
        const r = o.wasm_bindgen__convert__closures_____invoke__h64d2f1ae602a501f(e, t, n);
        if (r[1]) throw mt(r[0]);
    }
    function bt(e, t, n, r) {
        o.wasm_bindgen__convert__closures_____invoke__h022fdcc8ac41b2bf(e, t, n, r);
    }
    const q = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((e)=>o.__wbg_wasmepubextractor_free(e, 1));
    function x(e) {
        const t = o.__externref_table_alloc();
        return o.__wbindgen_externrefs.set(t, e), t;
    }
    const N = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((e)=>o.__wbindgen_destroy_closure(e.a, e.b));
    function V(e, t) {
        return e = e >>> 0, E().subarray(e / 1, e / 1 + t);
    }
    let p = null;
    function H() {
        return (p === null || p.buffer.detached === !0 || p.buffer.detached === void 0 && p.buffer !== o.memory.buffer) && (p = new DataView(o.memory.buffer)), p;
    }
    function C(e, t) {
        return yt(e >>> 0, t);
    }
    let v = null;
    function E() {
        return (v === null || v.byteLength === 0) && (v = new Uint8Array(o.memory.buffer)), v;
    }
    function wt(e, t) {
        try {
            return e.apply(this, t);
        } catch (n) {
            const r = x(n);
            o.__wbindgen_exn_store(r);
        }
    }
    function U(e) {
        return e == null;
    }
    function pt(e, t, n) {
        const r = {
            a: e,
            b: t,
            cnt: 1
        }, s = (...l)=>{
            r.cnt++;
            const a = r.a;
            r.a = 0;
            try {
                return n(a, r.b, ...l);
            } finally{
                r.a = a, s._wbg_cb_unref();
            }
        };
        return s._wbg_cb_unref = ()=>{
            --r.cnt === 0 && (o.__wbindgen_destroy_closure(r.a, r.b), r.a = 0, N.unregister(r));
        }, N.register(s, r, r), s;
    }
    function I(e, t, n) {
        if (n === void 0) {
            const c = A.encode(e), _ = t(c.length, 1) >>> 0;
            return E().subarray(_, _ + c.length).set(c), h = c.length, _;
        }
        let r = e.length, s = t(r, 1) >>> 0;
        const l = E();
        let a = 0;
        for(; a < r; a++){
            const c = e.charCodeAt(a);
            if (c > 127) break;
            l[s + a] = c;
        }
        if (a !== r) {
            a !== 0 && (e = e.slice(a)), s = n(s, r, r = a + e.length * 3, 1) >>> 0;
            const c = E().subarray(s + a, s + r), _ = A.encodeInto(e, c);
            a += _.written, s = n(s, r, a, 1) >>> 0;
        }
        return h = a, s;
    }
    function mt(e) {
        const t = o.__wbindgen_externrefs.get(e);
        return o.__externref_table_dealloc(e), t;
    }
    let O = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    O.decode();
    const ht = 2146435072;
    let F = 0;
    function yt(e, t) {
        return F += t, F >= ht && (O = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), O.decode(), F = t), O.decode(E().subarray(e, e + t));
    }
    const A = new TextEncoder;
    "encodeInto" in A || (A.encodeInto = function(e, t) {
        const n = A.encode(e);
        return t.set(n), {
            read: e.length,
            written: n.length
        };
    });
    let h = 0, o;
    function xt(e, t) {
        return o = e.exports, p = null, v = null, o.__wbindgen_start(), o;
    }
    async function vt(e, t) {
        if (typeof Response == "function" && e instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming == "function") try {
                return await WebAssembly.instantiateStreaming(e, t);
            } catch (s) {
                if (e.ok && n(e.type) && e.headers.get("Content-Type") !== "application/wasm") console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", s);
                else throw s;
            }
            const r = await e.arrayBuffer();
            return await WebAssembly.instantiate(r, t);
        } else {
            const r = await WebAssembly.instantiate(e, t);
            return r instanceof WebAssembly.Instance ? {
                instance: r,
                module: e
            } : r;
        }
        function n(r) {
            switch(r){
                case "basic":
                case "cors":
                case "default":
                    return !0;
            }
            return !1;
        }
    }
    async function Et(e) {
        if (o !== void 0) return o;
        e !== void 0 && (Object.getPrototypeOf(e) === Object.prototype ? { module_or_path: e } = e : console.warn("using deprecated parameters for the initialization function; pass a single object instead")), e === void 0 && (e = new URL("/Honzo/demo/assets/lexepub_bg-C9YBf47T.wasm", import.meta.url));
        const t = dt();
        (typeof e == "string" || typeof Request == "function" && e instanceof Request || typeof URL == "function" && e instanceof URL) && (e = fetch(e));
        const { instance: n, module: r } = await vt(await e, t);
        return xt(n);
    }
    const At = "/Honzo/demo/assets/lexepub_bg-C9YBf47T.wasm";
    let G = !1, X = !1;
    const m = document.getElementById("dropZone"), Y = document.getElementById("fileInput"), St = document.getElementById("status"), Tt = document.getElementById("statusText"), kt = document.getElementById("progressFill"), Rt = document.getElementById("historyList"), Z = B(S("convert", "statusVisible"), !1), K = B(S("convert", "statusKind"), ""), J = B(S("convert", "statusMessage"), ""), Q = B(S("convert", "progressWidth"), "0%"), Lt = ot(S("convert", "statusClass"), ()=>`status${Z.get() ? ` active ${K.get()}` : ""}`);
    st(St, Lt);
    at(Tt, J);
    ct(kt, "width", Q);
    T(m, "click", ()=>Y.click());
    T(m, "dragover", (e)=>{
        e.preventDefault(), m.classList.add("dragover");
    });
    T(m, "dragleave", ()=>m.classList.remove("dragover"));
    T(m, "drop", (e)=>{
        e.preventDefault(), m.classList.remove("dragover"), tt(e.dataTransfer?.files?.[0]);
    });
    T(Y, "change", (e)=>{
        tt(e.target.files?.[0]);
    });
    async function Mt() {
        G || (await Et(At), G = !0);
    }
    async function Ut() {
        X || (await it(), X = !0);
    }
    async function tt(e) {
        if (e) {
            w("loading", `Reading ${e.name}...`, 10);
            try {
                const t = await e.arrayBuffer();
                w("loading", "Parsing EPUB with lexepub...", 30), await Mt();
                const n = new P;
                await n.load_from_bytes(new Uint8Array(t)), w("loading", "Extracting metadata...", 50);
                const r = await n.get_metadata(), s = await n.get_toc();
                if (!Array.isArray(s) || s.length === 0) throw new Error("No chapters found in this EPUB");
                const a = ((i)=>i instanceof Map ? i.values().next().value : typeof i == "object" ? Object.values(i)[0] : i)(r?.title) || e.name.replace(/\.epub$/i, ""), c = Array.isArray(r?.authors) ? r.authors[0] : r?.creator || "Unknown", _ = Array.isArray(r?.languages) ? r.languages[0] : "en", f = [];
                for (const i of s){
                    const W = i.chapter_href;
                    if (W) try {
                        const g = await n.get_resource(W), y = new TextDecoder().decode(g).replace(/<script[\s\S]*?<\/script>/gi, "").replace(/<style[\s\S]*?<\/style>/gi, "");
                        f.push(y);
                    } catch  {
                        try {
                            const g = await n.get_chapter_text(i.chapter_index ?? f.length);
                            g && f.push(`<p>${Ot(g)}</p>`);
                        } catch  {}
                    }
                }
                if (f.length === 0) throw new Error("No readable chapters found in this EPUB");
                w("loading", "Extracting images and assets...", 60);
                const b = f.map((i)=>({
                        tag: "CHAP",
                        data: new TextEncoder().encode(i),
                        compression: 0,
                        content_type_kind: 1,
                        content_type_value: 1
                    }));
                try {
                    const i = await n.get_resource("META-INF/container.xml"), g = new DOMParser().parseFromString(new TextDecoder().decode(i), "text/xml").querySelector("rootfile")?.getAttribute("full-path");
                    if (g) {
                        const j = await n.get_resource(g), y = new DOMParser().parseFromString(new TextDecoder().decode(j), "text/xml"), et = g.includes("/") ? g.slice(0, g.lastIndexOf("/") + 1) : "", nt = (u)=>u.startsWith("/") ? u.slice(1) : et + u;
                        let D = null;
                        for (const u of y.querySelectorAll("meta"))u.getAttribute("name") === "cover" && (D = u.getAttribute("content"));
                        for (const u of y.querySelectorAll("item"))u.getAttribute("properties")?.includes("cover-image") && (D = u.getAttribute("id"));
                        for (const u of y.querySelectorAll("item")){
                            const R = u.getAttribute("id"), L = u.getAttribute("href"), M = u.getAttribute("media-type") || "";
                            if (!(!R || !L)) try {
                                const rt = nt(L), z = await n.get_resource(rt);
                                M.startsWith("image/") ? b.push({
                                    tag: R === D ? "COVR" : "IMG_",
                                    data: new Uint8Array(z),
                                    compression: 0,
                                    content_type_kind: 1,
                                    content_type_value: 0,
                                    alt_text: null
                                }) : M === "text/css" ? b.push({
                                    tag: "CSS_",
                                    data: new Uint8Array(z),
                                    compression: 0,
                                    content_type_kind: 1,
                                    content_type_value: 0,
                                    alt_text: null
                                }) : (M.startsWith("font/") || M.includes("font")) && b.push({
                                    tag: "FONT",
                                    data: new Uint8Array(z),
                                    compression: 0,
                                    content_type_kind: 1,
                                    content_type_value: 0,
                                    alt_text: null
                                });
                            } catch  {}
                        }
                        if (b.find((u)=>u.tag === "COVR")) try {
                            const { honzo_std: u } = await ft(async ()=>{
                                const { honzo_std: R } = await import("./honzo_wasm-B7nTe1NJ.js").then((L)=>L.i);
                                return {
                                    honzo_std: R
                                };
                            }, []);
                        } catch  {}
                    }
                } catch  {}
                w("loading", `Building .hzo (${b.length} chunks)...`, 70), await Ut();
                const d = {
                    chunks: b.map((i)=>({
                            tag: i.tag,
                            data: i.data,
                            compression: i.compression,
                            content_type_kind: i.content_type_kind,
                            content_type_value: i.content_type_value
                        })),
                    meta: {
                        title: {
                            [_]: a
                        },
                        authors: [
                            c
                        ],
                        language: _,
                        source_format: "epub"
                    }
                }, k = await _t(d);
                w("success", `Converted: ${k.length.toLocaleString()} bytes`, 100), It(a, k);
            } catch (t) {
                w("error", `Error: ${t?.message || t}`, 0);
            }
        }
    }
    function w(e, t, n) {
        Z.set(!0), K.set(e), J.set(t), Q.set(`${n}%`);
    }
    function It(e, t) {
        const n = document.createElement("div");
        n.className = "history-item";
        const r = e.replace(/[^a-zA-Z0-9_-]/g, "_") + ".hzo", s = new Blob([
            t
        ], {
            type: "application/octet-stream"
        }), l = URL.createObjectURL(s);
        n.innerHTML = `
    <div>
      <div class="name">${r}</div>
      <div class="size">${t.length.toLocaleString()} bytes</div>
    </div>
    <a class="dl-btn" href="${l}" download="${r}">Download</a>
  `, Rt.prepend(n);
    }
    function Ot(e) {
        return e.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    }
})();
