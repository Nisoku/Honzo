import{s as c,p as o,h as v,b,f as j,c as y,H as q,i as J,_ as W}from"./honzo_wasm-DNVA5GQH.js";let k=!1,l=null;const _=c(o("inspect","fileLoaded"),!1),C=c(o("inspect","fileName"),""),D=c(o("inspect","fileSize"),0),B=c(o("inspect","fileInfoData"),null),A=c(o("inspect","tocData"),[]),I=c(o("inspect","metaData"),null),h=c(o("inspect","extraData"),null),N=c(o("inspect","chunksData"),[]),z=c(o("inspect","originalMeta"),null),S=c(o("inspect","statusVisible"),!1),E=c(o("inspect","statusKind"),""),P=c(o("inspect","statusMessage"),""),K=v(o("inspect","statusClass"),()=>`status${S.get()?` active ${E.get()}`:""}`),G=v(o("inspect","filePanelClass"),()=>`panel${_.get()?" visible":""}`),Z=v(o("inspect","tocPanelClass"),()=>`panel${_.get()?" visible":""}`),Y=v(o("inspect","metaPanelClass"),()=>`panel${_.get()?" visible":""}`),Q=v(o("inspect","extraPanelClass"),()=>`panel${_.get()&&h.get()?.length>0?" visible":""}`),X=v(o("inspect","saveDisabled"),()=>!_.get()),ee=v(o("inspect","revertDisabled"),()=>!_.get()),f=document.getElementById("dropZone"),F=document.getElementById("fileInput"),te=document.getElementById("status"),ne=document.getElementById("statusText"),se=document.getElementById("filePanel"),ae=document.getElementById("fileInfo"),ie=document.getElementById("tocPanel"),le=document.getElementById("chunkCount"),oe=document.getElementById("tocBody"),re=document.getElementById("metaPanel"),$=document.getElementById("metaFields"),ce=document.getElementById("extraPanel"),w=document.getElementById("extraInfo"),H=document.getElementById("revertBtn"),O=document.getElementById("saveBtn");b(te,K);j(ne,P);b(se,G);b(ie,Z);b(re,Y);b(ce,Q);const U=(e,t)=>{const n=()=>e.disabled=t.get();n(),t.subscribe?.(n)??(t._onSet=n)};U(O,X);U(H,ee);y(f,"click",()=>F.click());y(f,"dragover",e=>{e.preventDefault(),f.classList.add("dragover")});y(f,"dragleave",()=>f.classList.remove("dragover"));y(f,"drop",e=>{e.preventDefault(),f.classList.remove("dragover"),R(e.dataTransfer?.files?.[0])});y(F,"change",e=>{e.target.files?.[0]&&R(e.target.files[0])});y(O,"click",ve);y(H,"click",ye);async function de(){k||(await W(),k=!0)}async function R(e){if(!e){d("error","No file selected");return}if(!e.name.endsWith(".hzo")){d("error","Please select a .hzo file");return}d("loading",`Loading ${e.name}...`);try{await de();const t=await e.arrayBuffer();l=new q(new Uint8Array(t),1),C.set(e.name),D.set(t.byteLength);const n=l.get_extra(),s=ue(l),a=l.get_meta_parsed();B.set({versionMajor:l.version_major(),versionMinor:l.version_minor(),minVer:l.min_reader_version(),flags:l.flags(),chunkCount:l.chunk_count(),tocSize:l.toc_size(),dataSize:l.data_size(),extraSize:l.extra_size(),metaSize:l.meta_size()}),A.set(l.get_toc()),I.set(a),z.set(JSON.parse(JSON.stringify(a))),h.set(n),N.set(s),me(),pe(),V(),ge(),_.set(!0),d("success",`Successfully loaded: ${e.name} (${m(t.byteLength)})`)}catch(t){console.error("Error loading file:",t),d("error",`Failed to load file: ${t.message||String(t)}`)}}function ue(e){return e.get_toc().map((n,s)=>({tag:n.chunk_type,data:Array.from(e.get_chunk(s)||new Uint8Array(0)),compression:n.compression,content_type_kind:n.content_type_kind,content_type_value:n.content_type_value,cover_type:n.cover_type,alt_text:n.alt_text||null,font_embedding:n.font_embedding,font_license_url:n.font_license_url||null}))}function me(){const e=B.get();if(!e||!l)return;const t=l.layout_mode_name(),n=l.compression_name(),s=(a,i)=>`<span class="flag-badge ${a?"on":"off"}">${a?"Yes":"No"}</span>`;ae.innerHTML=`
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
    <div class="info-item">
      <span class="label">Features</span>
      <div class="value" style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 0.5rem;">
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <span style="font-size: 0.85rem;">Search Index:</span>
          ${s(l.has_sidx())}
        </div>
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <span style="font-size: 0.85rem;">DRM:</span>
          ${s(l.has_drm())}
        </div>
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <span style="font-size: 0.85rem;">Annotations:</span>
          ${s(l.has_annotations())}
        </div>
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <span style="font-size: 0.85rem;">Sync:</span>
          ${s(l.has_sync())}
        </div>
      </div>
    </div>
  `}function pe(){const e=A.get(),t=B.get()?.chunkCount||0;le.textContent=`(${t} total)`,oe.innerHTML=e.map((n,s)=>{const a=typeof n.chunk_type=="string"?n.chunk_type:new TextDecoder().decode(new Uint8Array(n.chunk_type)),i=l.compression_name_for_chunk(s),u=l.content_type_name_for_chunk(s);return`<tr>
      <td>${s}</td>
      <td><strong>${p(a)}</strong></td>
      <td>${m(Number(n.size_compressed))}</td>
      <td>${m(Number(n.size_raw))}</td>
      <td>${i}</td>
      <td>${u}</td>
      <td>0x${n.flags.toString(16).padStart(4,"0")}</td>
    </tr>`}).join("")}function V(){const e=I.get();if(!e||typeof e!="object"){$.innerHTML="<p style='color:#888'>No metadata</p>";return}let t="";t+=r("Title","title",e.title,!0,"text"),t+=r("Subtitle","subtitle",e.subtitle,!0,"text"),t+=r("Authors","authors",e.authors,!1,"csv"),t+=r("Language","language",e.language,!1,"text"),t+=r("Publisher","publisher",e.publisher,!0,"text"),t+=r("Description","description",e.description,!0,"textarea"),t+=r("Source URL","source_url",e.source_url,!0,"text"),t+=r("License","license",e.license,!0,"text"),t+=r("Edition","edition",e.edition,!0,"text"),t+=r("Word Count","word_count",e.word_count,!0,"number"),t+=r("Reading Time (min)","reading_time_mins",e.reading_time_mins,!0,"number"),t+=L("Genres","genres",e.genres),t+=L("Tags","tags",e.tags),t+='<h3>Identifiers</h3><div id="idList">';const n=e.identifiers||[];t+=n.map((s,a)=>`
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${p(s.id_type||"")}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${p(s.value||"")}" /></div>
      <button class="btn btn-secondary" style="padding:0.4rem 0.6rem;font-size:0.8rem" onclick="this.parentElement.remove()">×</button>
    </div>`).join(""),t+='</div><button class="btn btn-secondary" style="font-size:0.85rem;margin-top:0.3rem" onclick="addIdentifier()">+ Add Identifier</button>',t+="<h3>Series</h3>",e.series?(t+=r("Series Title","series_title",e.series.title,!0,"text"),t+=r("Series Position","series_pos",e.series.position,!0,"text"),t+=r("Series Arc","series_arc",e.series.arc,!0,"text")):t+='<p style="color:#888;font-size:0.9rem">No series info</p>',$.innerHTML=t,window.addTag=function(s){const a=document.getElementById(`new_${s}`),i=a.value.trim();i&&(document.getElementById(`tags_${s}`).insertAdjacentHTML("beforeend",`<span class="tag"><span class="tag-text">${p(i)}</span> <span class="remove" onclick="this.parentElement.remove()">×</span></span>`),a.value="")},window.addIdentifier=function(){document.getElementById("idList").insertAdjacentHTML("beforeend",`
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" style="padding:0.4rem 0.6rem;font-size:0.8rem" onclick="this.parentElement.remove()">×</button>
      </div>`)},$.innerHTML=t}function r(e,t,n,s,a){const i=n??"",u=typeof i=="object"&&i!==null?Object.values(i)[0]||"":String(i),x=a==="textarea"?`<textarea id="mf_${t}">${p(u)}</textarea>`:a==="csv"?`<input type="text" id="mf_${t}" value="${p(Array.isArray(i)?i.join(", "):u)}" />`:`<input type="${a}" id="mf_${t}" value="${p(u)}" />`;return`<div class="field"><label for="mf_${t}">${e}${s?"":" *"}</label>${x}</div>`}function L(e,t,n){const s=Array.isArray(n)?n:[];return`<div class="field"><label>${e}</label>
    <div class="tag-list" id="tags_${t}">${s.map(a=>`<span class="tag"><span class="tag-text">${p(a)}</span> <span class="remove" onclick="this.parentElement.remove()">×</span></span>`).join("")}</div>
    <div class="tag-input"><input type="text" id="new_${t}" placeholder="Add ${e.toLowerCase()}" />
    <button onclick="addTag('${t}')">Add</button></div>
  </div>`}function ge(){const e=h.get();if(!e||e.length===0){w.innerHTML="<p style='color:#888'>No extra data</p>";return}w.innerHTML=`<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${m(e.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${be(e)}</pre>
  </details>`}function fe(){const e=JSON.parse(JSON.stringify(z.get()));g(e,"title",document.getElementById("mf_title")?.value),g(e,"subtitle",document.getElementById("mf_subtitle")?.value);const t=document.getElementById("mf_authors")?.value||"";e.authors=t.split(",").map(i=>i.trim()).filter(Boolean),e.language=document.getElementById("mf_language")?.value||"en",g(e,"publisher",document.getElementById("mf_publisher")?.value),g(e,"description",document.getElementById("mf_description")?.value),g(e,"source_url",document.getElementById("mf_source_url")?.value),g(e,"license",document.getElementById("mf_license")?.value),g(e,"edition",document.getElementById("mf_edition")?.value),T(e,"word_count",document.getElementById("mf_word_count")?.value),T(e,"reading_time_mins",document.getElementById("mf_reading_time_mins")?.value),e.genres=M("genres"),e.tags=M("tags");const n=document.querySelectorAll("#idList > div"),s=[];for(const i of n){const u=i.querySelector(".id-type")?.value?.trim(),x=i.querySelector(".id-value")?.value?.trim();u&&x&&s.push({id_type:u,value:x})}e.identifiers=s.length>0?s:void 0;const a=document.getElementById("mf_series_title")?.value?.trim();return a?e.series={title:a,position:document.getElementById("mf_series_pos")?.value?.trim()||void 0,arc:document.getElementById("mf_series_arc")?.value?.trim()||void 0}:e.series=void 0,e}function g(e,t,n){if(!n||!n.trim()){delete e[t];return}const s=n.trim();if(e[t]&&typeof e[t]=="object"&&!Array.isArray(e[t])){e[t]={...e[t]};const a=Object.keys(e[t]);a.length>0?e[t][a[0]]=s:e[t]={en:s}}else e[t]=s}function T(e,t,n){const s=parseInt(n,10);if(isNaN(s)){delete e[t];return}e[t]=s}function M(e){const t=document.getElementById("tags_"+e);if(!t)return;const n=[];for(const s of t.querySelectorAll(".tag")){const a=s.querySelector(".tag-text")?.textContent?.trim()||s.textContent.replace("×","").trim();a&&n.push(a)}return n.length>0?n:void 0}function ve(){if(!l){d("error","No file loaded to save");return}d("loading","Saving changes...");try{const e=fe(),t=N.get().map(i=>({tag:i.tag,data:new Uint8Array(i.data),compression:i.compression,content_type_kind:i.content_type_kind,content_type_value:i.content_type_value,alt_text:i.alt_text,font_embedding:i.font_embedding,font_license_url:i.font_license_url})),n=h.get(),s=J({chunks:t,meta:e,extra:n?.length?new Uint8Array(n):void 0,language:e.language||"en",auto_sidx:!0}),a=C.get().replace(/\.hzo$/i,"_edited.hzo");_e(s,a),d("success",`File saved successfully as ${a}`)}catch(e){console.error("Error saving file:",e),d("error",`Failed to save file: ${e.message||String(e)}`)}}function ye(){const e=z.get();e?(I.set(JSON.parse(JSON.stringify(e))),V(),d("success","Metadata reverted to original")):d("error","No original metadata to revert to")}function d(e,t){S.set(!0),E.set(e),P.set(t),(e==="success"||e==="loading")&&setTimeout(()=>{E.get()===e&&S.set(!1)},5e3)}function _e(e,t){const n=new Blob([e],{type:"application/octet-stream"}),s=URL.createObjectURL(n),a=document.createElement("a");a.href=s,a.download=t,a.click(),URL.revokeObjectURL(s)}function m(e){const t=Number(e);return t<1024?t+" B":t<1048576?(t/1024).toFixed(1)+" KB":t<1073741824?(t/1048576).toFixed(1)+" MB":(t/1073741824).toFixed(1)+" GB"}function p(e){return e?String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;"):""}function be(e){const t=new Uint8Array(e||[]);let n="";for(let s=0;s<Math.min(t.length,512);s++)s>0&&s%32===0&&(n+=`
`),n+=t[s].toString(16).padStart(2,"0")+" ";return t.length>512&&(n+=`
... (${t.length-512} more bytes)`),n}
//# sourceMappingURL=inspect-BXgLCYFG.js.map
