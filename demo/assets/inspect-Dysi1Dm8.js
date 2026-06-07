import{s as d,p as o,h as _,b as h,e as j,c as g,g as W,k as K,H as Z,i as G,_ as Y}from"./honzo_wasm-CU-UuUom.js";import{e as p}from"./esc-DSBv9wKl.js";import{f as m,d as Q}from"./download-wECEjfkj.js";let w=!1,l=null;const b=d(o("inspect","fileLoaded"),!1),A=d(o("inspect","fileName"),""),D=d(o("inspect","fileSize"),0),E=d(o("inspect","fileInfoData"),null),N=d(o("inspect","tocData"),[]),B=d(o("inspect","metaData"),null),x=d(o("inspect","extraData"),null),P=d(o("inspect","chunksData"),[]),z=d(o("inspect","originalMeta"),null),I=d(o("inspect","statusVisible"),!1),S=d(o("inspect","statusKind"),""),H=d(o("inspect","statusMessage"),""),X=_(o("inspect","statusClass"),()=>`status${I.get()?` active ${S.get()}`:""}`),ee=_(o("inspect","filePanelClass"),()=>`panel${b.get()?" visible":""}`),te=_(o("inspect","tocPanelClass"),()=>`panel${b.get()?" visible":""}`),ne=_(o("inspect","metaPanelClass"),()=>`panel${b.get()?" visible":""}`),se=_(o("inspect","extraPanelClass"),()=>`panel${b.get()&&x.get()?.length>0?" visible":""}`),ae=_(o("inspect","saveDisabled"),()=>!b.get()),ie=_(o("inspect","revertDisabled"),()=>!b.get()),y=document.getElementById("dropZone"),F=document.getElementById("fileInput"),le=document.getElementById("status"),oe=document.getElementById("statusText"),re=document.getElementById("filePanel"),de=document.getElementById("fileInfo"),ce=document.getElementById("tocPanel"),ue=document.getElementById("chunkCount"),me=document.getElementById("tocBody"),V=document.getElementById("metaPanel"),$=document.getElementById("metaFields"),pe=document.getElementById("extraPanel"),C=document.getElementById("extraInfo"),O=document.getElementById("revertBtn"),R=document.getElementById("saveBtn");h(le,X);j(oe,H);h(re,ee);h(ce,te);h(V,ne);h(pe,se);const U=(e,t)=>{const n=()=>e.disabled=t.get();n(),t.subscribe?.(n)??(t._onSet=n)};U(R,ae);U(O,ie);g(y,"click",()=>F.click());g(y,"dragover",e=>{e.preventDefault(),y.classList.add("dragover")});g(y,"dragleave",()=>y.classList.remove("dragover"));g(y,"drop",e=>{e.preventDefault(),y.classList.remove("dragover"),J(e.dataTransfer?.files?.[0])});g(F,"change",e=>{e.target.files?.[0]&&J(e.target.files[0])});g(R,"click",he);g(O,"click",xe);W({icons:K});g(V,"click",e=>{if(e.target.closest("[data-add-id]")){const a=document.getElementById("idList");a&&a.insertAdjacentHTML("beforeend",`
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
      </div>`)}const n=e.target.closest("[data-remove]");n&&n.closest(".field")?.remove();const s=e.target.closest("[data-tag-add]");if(s){const a=s.dataset.tagAdd,c=document.getElementById(`new_${a}`),v=c?.value?.trim();if(v){const k=document.getElementById(`tags_${a}`);k&&k.insertAdjacentHTML("beforeend",`<span class="tag"><span class="tag-text">${p(v)}</span> <span class="tag-remove" data-tag-id="${a}">×</span></span>`),c.value=""}}const i=e.target.closest(".tag-remove");i&&i.parentElement.remove()});async function ge(){w||(await Y(),w=!0)}async function J(e){if(!e){u("error","No file selected");return}if(!e.name.endsWith(".hzo")){u("error","Please select a .hzo file");return}u("loading",`Loading ${e.name}...`);try{await ge();const t=await e.arrayBuffer();l=new Z(new Uint8Array(t),1),A.set(e.name),D.set(t.byteLength);const n=l.get_extra(),s=ve(l),i=l.get_meta_parsed();E.set({versionMajor:l.version_major(),versionMinor:l.version_minor(),minVer:l.min_reader_version(),flags:l.flags(),chunkCount:l.chunk_count(),tocSize:l.toc_size(),dataSize:l.data_size(),extraSize:l.extra_size(),metaSize:l.meta_size()}),N.set(l.get_toc()),B.set(i),z.set(JSON.parse(JSON.stringify(i))),x.set(n),P.set(s),fe(),ye(),q(),_e(),b.set(!0),u("success",`Successfully loaded: ${e.name} (${m(t.byteLength)})`)}catch(t){console.error("Error loading file:",t),u("error",`Failed to load file: ${t.message||String(t)}`)}}function ve(e){return e.get_toc().map((n,s)=>({tag:n.chunk_type,data:Array.from(e.get_chunk(s)||new Uint8Array(0)),compression:n.compression,content_type_kind:n.content_type_kind,content_type_value:n.content_type_value,cover_type:n.cover_type,alt_text:n.alt_text||null,font_embedding:n.font_embedding,font_license_url:n.font_license_url||null}))}function fe(){const e=E.get();if(!e||!l)return;const t=l.layout_mode_name(),n=l.compression_name(),s=(i,a)=>`<span class="flag-badge ${i?"on":"off"}">${i?"Yes":"No"}</span>`;de.innerHTML=`
    <div class="info-grid">
      <div class="info-item">
        <span class="label">File Size</span>
        <div class="value">${m(D.get())}</div>
      </div>
      <div class="info-item">
        <span class="label">Format Version</span>
        <div class="value">${e.versionMajor}.${e.versionMinor}</div>
      </div>
      <div class="info-item">
        <span class="label">Min Reader Version</span>
        <div class="value">${e.minVer}</div>
      </div>
      <div class="info-item">
        <span class="label">Chunks</span>
        <div class="value">${e.chunkCount}</div>
      </div>
      <div class="info-item">
        <span class="label">Layout Mode</span>
        <div class="value">${t}</div>
      </div>
      <div class="info-item">
        <span class="label">Default Compression</span>
        <div class="value">${n}</div>
      </div>
      <div class="info-item">
        <span class="label">TOC Size</span>
        <div class="value">${m(e.tocSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Data Size</span>
        <div class="value">${m(e.dataSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Extra Data Size</span>
        <div class="value">${m(e.extraSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Metadata Size</span>
        <div class="value">${m(e.metaSize)}</div>
      </div>
      <div class="info-item" style="grid-column: 1 / -1">
        <span class="label">Features</span>
        <div class="value features-grid">
          <div>
            <span>Search Index:</span>
            ${s(l.has_sidx())}
          </div>
          <div>
            <span>DRM:</span>
            ${s(l.has_drm())}
          </div>
          <div>
            <span>Annotations:</span>
            ${s(l.has_annotations())}
          </div>
          <div>
            <span>Sync:</span>
            ${s(l.has_sync())}
          </div>
        </div>
      </div>
    </div>
  `}function ye(){const e=N.get(),t=E.get()?.chunkCount||0;ue.textContent=`(${t} total)`,me.innerHTML=e.map((n,s)=>{const i=typeof n.chunk_type=="string"?n.chunk_type:new TextDecoder().decode(new Uint8Array(n.chunk_type)),a=l.compression_name_for_chunk(s),c=l.content_type_name_for_chunk(s);return`<tr>
      <td>${s}</td>
      <td><strong>${p(i)}</strong></td>
      <td>${m(Number(n.size_compressed))}</td>
      <td>${m(Number(n.size_raw))}</td>
      <td>${a}</td>
      <td>${c}</td>
      <td>0x${n.flags.toString(16).padStart(4,"0")}</td>
    </tr>`}).join("")}function q(){const e=B.get();if(!e||typeof e!="object"){$.innerHTML="<p style='color:#888'>No metadata</p>";return}let t="";t+=r("Title","title",e.title,!0,"text"),t+=r("Subtitle","subtitle",e.subtitle,!0,"text"),t+=r("Authors","authors",e.authors,!1,"csv"),t+=r("Language","language",e.language,!1,"text"),t+=r("Publisher","publisher",e.publisher,!0,"text"),t+=r("Description","description",e.description,!0,"textarea"),t+=r("Source URL","source_url",e.source_url,!0,"text"),t+=r("License","license",e.license,!0,"text"),t+=r("Edition","edition",e.edition,!0,"text"),t+=r("Word Count","word_count",e.word_count,!0,"number"),t+=r("Reading Time (min)","reading_time_mins",e.reading_time_mins,!0,"number"),t+=L("Genres","genres",e.genres),t+=L("Tags","tags",e.tags),t+='<h3>Identifiers</h3><div id="idList">';const n=e.identifiers||[];t+=n.map((s,i)=>`
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${p(s.id_type||"")}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${p(s.value||"")}" /></div>
      <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
    </div>`).join(""),t+='</div><button class="btn btn-secondary" data-add-id="1" style="font-size:0.85rem;margin-top:0.3rem">+ Add Identifier</button>',t+="<h3>Series</h3>",e.series?(t+=r("Series Title","series_title",e.series.title,!0,"text"),t+=r("Series Position","series_pos",e.series.position,!0,"text"),t+=r("Series Arc","series_arc",e.series.arc,!0,"text")):t+='<p style="color:#888;font-size:0.9rem">No series info</p>',$.innerHTML=t,$.innerHTML=t}function r(e,t,n,s,i){const a=n??"",c=typeof a=="object"&&a!==null?Object.values(a)[0]||"":String(a),v=i==="textarea"?`<textarea id="mf_${t}">${p(c)}</textarea>`:i==="csv"?`<input type="text" id="mf_${t}" value="${p(Array.isArray(a)?a.join(", "):c)}" />`:`<input type="${i}" id="mf_${t}" value="${p(c)}" />`;return`<div class="field"><label for="mf_${t}">${e}${s?"":" *"}</label>${v}</div>`}function L(e,t,n){const s=Array.isArray(n)?n:[];return`<div class="field"><label>${e}</label>
    <div class="tag-list" id="tags_${t}">${s.map(i=>`<span class="tag"><span class="tag-text">${p(i)}</span> <span class="tag-remove" data-tag-id="${t}">×</span></span>`).join("")}</div>
    <div class="tag-input"><input type="text" id="new_${t}" placeholder="Add ${e.toLowerCase()}" />
    <button data-tag-add="${t}">Add</button></div>
  </div>`}function _e(){const e=x.get();if(!e||e.length===0){C.innerHTML="<p style='color:#888'>No extra data</p>";return}C.innerHTML=`<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${m(e.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${$e(e)}</pre>
  </details>`}function be(){const e=JSON.parse(JSON.stringify(z.get()));f(e,"title",document.getElementById("mf_title")?.value),f(e,"subtitle",document.getElementById("mf_subtitle")?.value);const t=document.getElementById("mf_authors")?.value||"";e.authors=t.split(",").map(a=>a.trim()).filter(Boolean),e.language=document.getElementById("mf_language")?.value||"en",f(e,"publisher",document.getElementById("mf_publisher")?.value),f(e,"description",document.getElementById("mf_description")?.value),f(e,"source_url",document.getElementById("mf_source_url")?.value),f(e,"license",document.getElementById("mf_license")?.value),f(e,"edition",document.getElementById("mf_edition")?.value),M(e,"word_count",document.getElementById("mf_word_count")?.value),M(e,"reading_time_mins",document.getElementById("mf_reading_time_mins")?.value),e.genres=T("genres"),e.tags=T("tags");const n=document.querySelectorAll("#idList > div"),s=[];for(const a of n){const c=a.querySelector(".id-type")?.value?.trim(),v=a.querySelector(".id-value")?.value?.trim();c&&v&&s.push({id_type:c,value:v})}e.identifiers=s.length>0?s:void 0;const i=document.getElementById("mf_series_title")?.value?.trim();return i?e.series={title:i,position:document.getElementById("mf_series_pos")?.value?.trim()||void 0,arc:document.getElementById("mf_series_arc")?.value?.trim()||void 0}:e.series=void 0,e}function f(e,t,n){if(!n||!n.trim()){delete e[t];return}const s=n.trim();if(e[t]&&typeof e[t]=="object"&&!Array.isArray(e[t])){e[t]={...e[t]};const i=Object.keys(e[t]);i.length>0?e[t][i[0]]=s:e[t]={en:s}}else e[t]=s}function M(e,t,n){const s=parseInt(n,10);if(isNaN(s)){delete e[t];return}e[t]=s}function T(e){const t=document.getElementById("tags_"+e);if(!t)return;const n=[];for(const s of t.querySelectorAll(".tag")){const i=s.querySelector(".tag-text")?.textContent?.trim()||s.textContent.replace("×","").trim();i&&n.push(i)}return n.length>0?n:void 0}function he(){if(!l){u("error","No file loaded to save");return}u("loading","Saving changes...");try{const e=be(),t=P.get().map(a=>({tag:a.tag,data:new Uint8Array(a.data),compression:a.compression,content_type_kind:a.content_type_kind,content_type_value:a.content_type_value,alt_text:a.alt_text,font_embedding:a.font_embedding,font_license_url:a.font_license_url})),n=x.get(),s=G({chunks:t,meta:e,extra:n?.length?new Uint8Array(n):void 0,language:e.language||"en",auto_sidx:!0}),i=A.get().replace(/\.hzo$/i,"_edited.hzo");Q(s,i),u("success",`File saved successfully as ${i}`)}catch(e){console.error("Error saving file:",e),u("error",`Failed to save file: ${e.message||String(e)}`)}}function xe(){const e=z.get();e?(B.set(JSON.parse(JSON.stringify(e))),q(),u("success","Metadata reverted to original")):u("error","No original metadata to revert to")}function u(e,t){I.set(!0),S.set(e),H.set(t),(e==="success"||e==="loading")&&setTimeout(()=>{S.get()===e&&I.set(!1)},5e3)}function $e(e){const t=new Uint8Array(e||[]);let n="";for(let s=0;s<Math.min(t.length,512);s++)s>0&&s%32===0&&(n+=`
`),n+=t[s].toString(16).padStart(2,"0")+" ";return t.length>512&&(n+=`
... (${t.length-512} more bytes)`),n}
//# sourceMappingURL=inspect-Dysi1Dm8.js.map
