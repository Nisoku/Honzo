import{s as c,p as o,h as _,b as x,e as J,c as g,g as W,k as K,H as G,i as Z,_ as Y}from"./honzo_wasm-BnLbK6IE.js";let L=!1,l=null;const b=c(o("inspect","fileLoaded"),!1),A=c(o("inspect","fileName"),""),D=c(o("inspect","fileSize"),0),I=c(o("inspect","fileInfoData"),null),N=c(o("inspect","tocData"),[]),E=c(o("inspect","metaData"),null),h=c(o("inspect","extraData"),null),P=c(o("inspect","chunksData"),[]),z=c(o("inspect","originalMeta"),null),S=c(o("inspect","statusVisible"),!1),B=c(o("inspect","statusKind"),""),F=c(o("inspect","statusMessage"),""),Q=_(o("inspect","statusClass"),()=>`status${S.get()?` active ${B.get()}`:""}`),X=_(o("inspect","filePanelClass"),()=>`panel${b.get()?" visible":""}`),ee=_(o("inspect","tocPanelClass"),()=>`panel${b.get()?" visible":""}`),te=_(o("inspect","metaPanelClass"),()=>`panel${b.get()?" visible":""}`),ne=_(o("inspect","extraPanelClass"),()=>`panel${b.get()&&h.get()?.length>0?" visible":""}`),se=_(o("inspect","saveDisabled"),()=>!b.get()),ae=_(o("inspect","revertDisabled"),()=>!b.get()),y=document.getElementById("dropZone"),H=document.getElementById("fileInput"),ie=document.getElementById("status"),le=document.getElementById("statusText"),oe=document.getElementById("filePanel"),re=document.getElementById("fileInfo"),ce=document.getElementById("tocPanel"),de=document.getElementById("chunkCount"),ue=document.getElementById("tocBody"),O=document.getElementById("metaPanel"),$=document.getElementById("metaFields"),me=document.getElementById("extraPanel"),w=document.getElementById("extraInfo"),R=document.getElementById("revertBtn"),U=document.getElementById("saveBtn");x(ie,Q);J(le,F);x(oe,X);x(ce,ee);x(O,te);x(me,ne);const V=(e,t)=>{const n=()=>e.disabled=t.get();n(),t.subscribe?.(n)??(t._onSet=n)};V(U,se);V(R,ae);g(y,"click",()=>H.click());g(y,"dragover",e=>{e.preventDefault(),y.classList.add("dragover")});g(y,"dragleave",()=>y.classList.remove("dragover"));g(y,"drop",e=>{e.preventDefault(),y.classList.remove("dragover"),j(e.dataTransfer?.files?.[0])});g(H,"change",e=>{e.target.files?.[0]&&j(e.target.files[0])});g(U,"click",be);g(R,"click",xe);W({icons:K});g(O,"click",e=>{if(e.target.closest("[data-add-id]")){const i=document.getElementById("idList");i&&i.insertAdjacentHTML("beforeend",`
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
      </div>`)}const n=e.target.closest("[data-remove]");n&&n.closest(".field")?.remove();const s=e.target.closest("[data-tag-add]");if(s){const i=s.dataset.tagAdd,d=document.getElementById(`new_${i}`),f=d?.value?.trim();if(f){const k=document.getElementById(`tags_${i}`);k&&k.insertAdjacentHTML("beforeend",`<span class="tag"><span class="tag-text">${p(f)}</span> <span class="tag-remove" data-tag-id="${i}">×</span></span>`),d.value=""}}const a=e.target.closest(".tag-remove");a&&a.parentElement.remove()});async function pe(){L||(await Y(),L=!0)}async function j(e){if(!e){u("error","No file selected");return}if(!e.name.endsWith(".hzo")){u("error","Please select a .hzo file");return}u("loading",`Loading ${e.name}...`);try{await pe();const t=await e.arrayBuffer();l=new G(new Uint8Array(t),1),A.set(e.name),D.set(t.byteLength);const n=l.get_extra(),s=ge(l),a=l.get_meta_parsed();I.set({versionMajor:l.version_major(),versionMinor:l.version_minor(),minVer:l.min_reader_version(),flags:l.flags(),chunkCount:l.chunk_count(),tocSize:l.toc_size(),dataSize:l.data_size(),extraSize:l.extra_size(),metaSize:l.meta_size()}),N.set(l.get_toc()),E.set(a),z.set(JSON.parse(JSON.stringify(a))),h.set(n),P.set(s),fe(),ve(),q(),ye(),b.set(!0),u("success",`Successfully loaded: ${e.name} (${m(t.byteLength)})`)}catch(t){console.error("Error loading file:",t),u("error",`Failed to load file: ${t.message||String(t)}`)}}function ge(e){return e.get_toc().map((n,s)=>({tag:n.chunk_type,data:Array.from(e.get_chunk(s)||new Uint8Array(0)),compression:n.compression,content_type_kind:n.content_type_kind,content_type_value:n.content_type_value,cover_type:n.cover_type,alt_text:n.alt_text||null,font_embedding:n.font_embedding,font_license_url:n.font_license_url||null}))}function fe(){const e=I.get();if(!e||!l)return;const t=l.layout_mode_name(),n=l.compression_name(),s=(a,i)=>`<span class="flag-badge ${a?"on":"off"}">${a?"Yes":"No"}</span>`;re.innerHTML=`
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
  `}function ve(){const e=N.get(),t=I.get()?.chunkCount||0;de.textContent=`(${t} total)`,ue.innerHTML=e.map((n,s)=>{const a=typeof n.chunk_type=="string"?n.chunk_type:new TextDecoder().decode(new Uint8Array(n.chunk_type)),i=l.compression_name_for_chunk(s),d=l.content_type_name_for_chunk(s);return`<tr>
      <td>${s}</td>
      <td><strong>${p(a)}</strong></td>
      <td>${m(Number(n.size_compressed))}</td>
      <td>${m(Number(n.size_raw))}</td>
      <td>${i}</td>
      <td>${d}</td>
      <td>0x${n.flags.toString(16).padStart(4,"0")}</td>
    </tr>`}).join("")}function q(){const e=E.get();if(!e||typeof e!="object"){$.innerHTML="<p style='color:#888'>No metadata</p>";return}let t="";t+=r("Title","title",e.title,!0,"text"),t+=r("Subtitle","subtitle",e.subtitle,!0,"text"),t+=r("Authors","authors",e.authors,!1,"csv"),t+=r("Language","language",e.language,!1,"text"),t+=r("Publisher","publisher",e.publisher,!0,"text"),t+=r("Description","description",e.description,!0,"textarea"),t+=r("Source URL","source_url",e.source_url,!0,"text"),t+=r("License","license",e.license,!0,"text"),t+=r("Edition","edition",e.edition,!0,"text"),t+=r("Word Count","word_count",e.word_count,!0,"number"),t+=r("Reading Time (min)","reading_time_mins",e.reading_time_mins,!0,"number"),t+=M("Genres","genres",e.genres),t+=M("Tags","tags",e.tags),t+='<h3>Identifiers</h3><div id="idList">';const n=e.identifiers||[];t+=n.map((s,a)=>`
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${p(s.id_type||"")}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${p(s.value||"")}" /></div>
      <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
    </div>`).join(""),t+='</div><button class="btn btn-secondary" data-add-id="1" style="font-size:0.85rem;margin-top:0.3rem">+ Add Identifier</button>',t+="<h3>Series</h3>",e.series?(t+=r("Series Title","series_title",e.series.title,!0,"text"),t+=r("Series Position","series_pos",e.series.position,!0,"text"),t+=r("Series Arc","series_arc",e.series.arc,!0,"text")):t+='<p style="color:#888;font-size:0.9rem">No series info</p>',$.innerHTML=t,$.innerHTML=t}function r(e,t,n,s,a){const i=n??"",d=typeof i=="object"&&i!==null?Object.values(i)[0]||"":String(i),f=a==="textarea"?`<textarea id="mf_${t}">${p(d)}</textarea>`:a==="csv"?`<input type="text" id="mf_${t}" value="${p(Array.isArray(i)?i.join(", "):d)}" />`:`<input type="${a}" id="mf_${t}" value="${p(d)}" />`;return`<div class="field"><label for="mf_${t}">${e}${s?"":" *"}</label>${f}</div>`}function M(e,t,n){const s=Array.isArray(n)?n:[];return`<div class="field"><label>${e}</label>
    <div class="tag-list" id="tags_${t}">${s.map(a=>`<span class="tag"><span class="tag-text">${p(a)}</span> <span class="tag-remove" data-tag-id="${t}">×</span></span>`).join("")}</div>
    <div class="tag-input"><input type="text" id="new_${t}" placeholder="Add ${e.toLowerCase()}" />
    <button data-tag-add="${t}">Add</button></div>
  </div>`}function ye(){const e=h.get();if(!e||e.length===0){w.innerHTML="<p style='color:#888'>No extra data</p>";return}w.innerHTML=`<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${m(e.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${$e(e)}</pre>
  </details>`}function _e(){const e=JSON.parse(JSON.stringify(z.get()));v(e,"title",document.getElementById("mf_title")?.value),v(e,"subtitle",document.getElementById("mf_subtitle")?.value);const t=document.getElementById("mf_authors")?.value||"";e.authors=t.split(",").map(i=>i.trim()).filter(Boolean),e.language=document.getElementById("mf_language")?.value||"en",v(e,"publisher",document.getElementById("mf_publisher")?.value),v(e,"description",document.getElementById("mf_description")?.value),v(e,"source_url",document.getElementById("mf_source_url")?.value),v(e,"license",document.getElementById("mf_license")?.value),v(e,"edition",document.getElementById("mf_edition")?.value),C(e,"word_count",document.getElementById("mf_word_count")?.value),C(e,"reading_time_mins",document.getElementById("mf_reading_time_mins")?.value),e.genres=T("genres"),e.tags=T("tags");const n=document.querySelectorAll("#idList > div"),s=[];for(const i of n){const d=i.querySelector(".id-type")?.value?.trim(),f=i.querySelector(".id-value")?.value?.trim();d&&f&&s.push({id_type:d,value:f})}e.identifiers=s.length>0?s:void 0;const a=document.getElementById("mf_series_title")?.value?.trim();return a?e.series={title:a,position:document.getElementById("mf_series_pos")?.value?.trim()||void 0,arc:document.getElementById("mf_series_arc")?.value?.trim()||void 0}:e.series=void 0,e}function v(e,t,n){if(!n||!n.trim()){delete e[t];return}const s=n.trim();if(e[t]&&typeof e[t]=="object"&&!Array.isArray(e[t])){e[t]={...e[t]};const a=Object.keys(e[t]);a.length>0?e[t][a[0]]=s:e[t]={en:s}}else e[t]=s}function C(e,t,n){const s=parseInt(n,10);if(isNaN(s)){delete e[t];return}e[t]=s}function T(e){const t=document.getElementById("tags_"+e);if(!t)return;const n=[];for(const s of t.querySelectorAll(".tag")){const a=s.querySelector(".tag-text")?.textContent?.trim()||s.textContent.replace("×","").trim();a&&n.push(a)}return n.length>0?n:void 0}function be(){if(!l){u("error","No file loaded to save");return}u("loading","Saving changes...");try{const e=_e(),t=P.get().map(i=>({tag:i.tag,data:new Uint8Array(i.data),compression:i.compression,content_type_kind:i.content_type_kind,content_type_value:i.content_type_value,alt_text:i.alt_text,font_embedding:i.font_embedding,font_license_url:i.font_license_url})),n=h.get(),s=Z({chunks:t,meta:e,extra:n?.length?new Uint8Array(n):void 0,language:e.language||"en",auto_sidx:!0}),a=A.get().replace(/\.hzo$/i,"_edited.hzo");he(s,a),u("success",`File saved successfully as ${a}`)}catch(e){console.error("Error saving file:",e),u("error",`Failed to save file: ${e.message||String(e)}`)}}function xe(){const e=z.get();e?(E.set(JSON.parse(JSON.stringify(e))),q(),u("success","Metadata reverted to original")):u("error","No original metadata to revert to")}function u(e,t){S.set(!0),B.set(e),F.set(t),(e==="success"||e==="loading")&&setTimeout(()=>{B.get()===e&&S.set(!1)},5e3)}function he(e,t){const n=new Blob([e],{type:"application/octet-stream"}),s=URL.createObjectURL(n),a=document.createElement("a");a.href=s,a.download=t,a.click(),URL.revokeObjectURL(s)}function m(e){const t=Number(e);return t<1024?t+" B":t<1048576?(t/1024).toFixed(1)+" KB":t<1073741824?(t/1048576).toFixed(1)+" MB":(t/1073741824).toFixed(1)+" GB"}function p(e){return e?String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;"):""}function $e(e){const t=new Uint8Array(e||[]);let n="";for(let s=0;s<Math.min(t.length,512);s++)s>0&&s%32===0&&(n+=`
`),n+=t[s].toString(16).padStart(2,"0")+" ";return t.length>512&&(n+=`
... (${t.length-512} more bytes)`),n}
//# sourceMappingURL=inspect-C7ZcT-t0.js.map
