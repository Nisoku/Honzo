import{_ as e,c as t,f as n,g as r,h as i,i as a,l as o,m as s,n as c,s as l,t as u,u as d}from"./honzo_wasm-BmfirpUZ.js";import{t as f}from"./esc-By3oFyRa.js";import{n as p,t as ee}from"./download-DA2FER3_.js";function te(e){return e instanceof Map?[...e.entries()]:e&&typeof e==`object`&&!Array.isArray(e)?Object.entries(e):[]}function m(e,t,n){let r=t==null?``:String(t);return n===`textarea`?`<textarea id="${e}">${f(r)}</textarea>`:`<input type="${n}" id="${e}" value="${f(r)}" />`}function h(e,t,n,r,i){let a=te(n);if(a.length>1)return`<div class="field">${a.map(([n,r],a)=>{let o=a===0?`mf_${t}`:`mf_${t}__${n}`,s=a===0?``:` (${f(n)})`;return`<label>${f(e)}${s}</label>${m(o,r,i)}`}).join(``)}</div>`;let o=n instanceof Map?n.values().next().value??``:n??``,s=typeof o==`object`&&o?Object.values(o)[0]??``:String(o),c=i===`csv`?`<input type="text" id="mf_${t}" value="${f(Array.isArray(o)?o.join(`, `):s)}" />`:m(t,s,i),l=r?``:` *`;return`<div class="field"><label for="mf_${t}">${f(e)}${l}</label>${c}</div>`}function ne(e,t,n){if(!n||!n.trim()){delete e[t];return}let r=n.trim();if(e[t]instanceof Map){let n=new Map(e[t]),i=n.keys().next().value;i===void 0?n.set(`en`,r):n.set(i,r),e[t]=n;return}if(e[t]&&typeof e[t]==`object`&&!Array.isArray(e[t])){e[t]={...e[t]};let n=Object.keys(e[t]);n.length>0?e[t][n[0]]=r:e[t]={en:r}}else e[t]=r}var g=!1,_=null,v=e(r(`inspect`,`fileLoaded`),!1),y=e(r(`inspect`,`fileName`),``),b=e(r(`inspect`,`fileSize`),0),x=e(r(`inspect`,`fileInfoData`),null),S=e(r(`inspect`,`tocData`),[]),C=e(r(`inspect`,`metaData`),null),w=e(r(`inspect`,`extraData`),null),T=e(r(`inspect`,`chunksData`),[]),E=e(r(`inspect`,`originalMeta`),null),D=e(r(`inspect`,`statusVisible`),!1),O=e(r(`inspect`,`statusKind`),``),k=e(r(`inspect`,`statusMessage`),``),A=i(r(`inspect`,`statusClass`),()=>`status${D.get()?` active ${O.get()}`:``}`),j=i(r(`inspect`,`filePanelClass`),()=>`panel${v.get()?` visible`:``}`),M=i(r(`inspect`,`tocPanelClass`),()=>`panel${v.get()?` visible`:``}`),re=i(r(`inspect`,`metaPanelClass`),()=>`panel${v.get()?` visible`:``}`),ie=i(r(`inspect`,`extraPanelClass`),()=>`panel${v.get()&&w.get()?.length>0?` visible`:``}`),ae=i(r(`inspect`,`saveDisabled`),()=>!v.get()),oe=i(r(`inspect`,`revertDisabled`),()=>!v.get()),N=document.getElementById(`dropZone`),P=document.getElementById(`fileInput`),se=document.getElementById(`status`),ce=document.getElementById(`statusText`),F=document.getElementById(`filePanel`),I=document.getElementById(`fileInfo`),L=document.getElementById(`tocPanel`),R=document.getElementById(`chunkCount`),z=document.getElementById(`tocBody`),B=document.getElementById(`metaPanel`),V=document.getElementById(`metaFields`),H=document.getElementById(`extraPanel`),U=document.getElementById(`extraInfo`),W=document.getElementById(`revertBtn`),G=document.getElementById(`saveBtn`);d(se,A),s(ce,k),d(F,j),d(L,M),d(B,re),d(H,ie);var K=(e,t)=>{let n=()=>e.disabled=t.get();n(),t.subscribe?.(n)??(t._onSet=n)};K(G,ae),K(W,oe),n(N,`click`,()=>P.click()),n(N,`dragover`,e=>{e.preventDefault(),N.classList.add(`dragover`)}),n(N,`dragleave`,()=>N.classList.remove(`dragover`)),n(N,`drop`,e=>{e.preventDefault(),N.classList.remove(`dragover`),q(e.dataTransfer?.files?.[0])}),n(P,`change`,e=>{e.target.files?.[0]&&q(e.target.files[0])}),n(G,`click`,he),n(W,`click`,ge),o({icons:t}),n(B,`click`,e=>{if(e.target.closest(`[data-add-id]`)){let e=document.getElementById(`idList`);e&&e.insertAdjacentHTML(`beforeend`,`
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" data-remove="inspect-id" aria-label="Remove identifier" style="padding:0.4rem 0.6rem;font-size:0.8rem">${l(`X`,14,2,{"aria-hidden":`true`})}</button>
      </div>`)}let t=e.target.closest(`[data-remove]`);t&&t.closest(`.field`)?.remove();let n=e.target.closest(`[data-tag-add]`);if(n){let e=n.dataset.tagAdd,t=document.getElementById(`new_${e}`),r=t?.value?.trim();if(r){let n=document.getElementById(`tags_${e}`);n&&n.insertAdjacentHTML(`beforeend`,`<span class="tag"><span class="tag-text">${f(r)}</span> <span class="tag-remove" data-tag-id="${e}">${l(`X`,12)}</span></span>`),t.value=``}}let r=e.target.closest(`.tag-remove`);r&&r.parentElement.remove()});async function le(){g||=(await c(),!0)}async function q(e){if(!e){$(`error`,`No file selected`);return}if(!e.name.endsWith(`.hzo`)){$(`error`,`Please select a .hzo file`);return}$(`loading`,`Loading ${e.name}...`);try{await le();let t=await e.arrayBuffer();_=new u(new Uint8Array(t),1),y.set(e.name),b.set(t.byteLength);let n=_.get_extra(),r=ue(_),i=_.get_meta_parsed();x.set({versionMajor:_.version_major(),versionMinor:_.version_minor(),minVer:_.min_reader_version(),flags:_.flags(),chunkCount:_.chunk_count(),tocSize:_.toc_size(),dataSize:_.data_size(),extraSize:_.extra_size(),metaSize:_.meta_size()}),S.set(_.get_toc()),C.set(i),E.set(structuredClone(i)),w.set(n),T.set(r),de(),fe(),J(),pe(),v.set(!0),$(`success`,`Successfully loaded: ${e.name} (${p(t.byteLength)})`)}catch(e){console.error(`Error loading file:`,e),$(`error`,`Failed to load file: ${e.message||String(e)}`)}}function ue(e){return e.get_toc().map((t,n)=>({tag:t.chunk_type,data:Array.from(e.get_chunk(n)||new Uint8Array),compression:t.compression,content_type_kind:t.content_type_kind,content_type_value:t.content_type_value,cover_type:t.cover_type,alt_text:t.alt_text||null,font_embedding:t.font_embedding,font_license_url:t.font_license_url||null}))}function de(){let e=x.get();if(!e||!_)return;let t=_.layout_mode_name(),n=_.compression_name(),r=(e,t)=>`<span class="flag-badge ${e?`on`:`off`}">${e?`Yes`:`No`}</span>`;I.innerHTML=`
    <div class="info-grid">
      <div class="info-item">
        <span class="label">File Size</span>
        <div class="value">${p(b.get())}</div>
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
        <div class="value">${p(e.tocSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Data Size</span>
        <div class="value">${p(e.dataSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Extra Data Size</span>
        <div class="value">${p(e.extraSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Metadata Size</span>
        <div class="value">${p(e.metaSize)}</div>
      </div>
      <div class="info-item" style="grid-column: 1 / -1">
        <span class="label">Features</span>
        <div class="value features-grid">
          <div>
            <span>Search Index:</span>
            ${r(_.has_sidx(),`Search Index`)}
          </div>
          <div>
            <span>DRM:</span>
            ${r(_.has_drm(),`DRM`)}
          </div>
          <div>
            <span>Annotations:</span>
            ${r(_.has_annotations(),`Annotations`)}
          </div>
          <div>
            <span>Sync:</span>
            ${r(_.has_sync(),`Sync`)}
          </div>
        </div>
      </div>
    </div>
  `}function fe(){let e=S.get();R.textContent=`(${x.get()?.chunkCount||0} total)`,z.innerHTML=e.map((e,t)=>{let n=typeof e.chunk_type==`string`?e.chunk_type:new TextDecoder().decode(new Uint8Array(e.chunk_type)),r=_.compression_name_for_chunk(t),i=_.content_type_name_for_chunk(t);return`<tr>
      <td>${t}</td>
      <td><strong>${f(n)}</strong></td>
      <td>${p(Number(e.size_compressed))}</td>
      <td>${p(Number(e.size_raw))}</td>
      <td>${r}</td>
      <td>${i}</td>
      <td>0x${e.flags.toString(16).padStart(4,`0`)}</td>
    </tr>`}).join(``)}function J(){let e=C.get();if(!e||typeof e!=`object`){V.innerHTML=`<p style='color:#888'>No metadata</p>`;return}let t=``;t+=h(`Title`,`title`,e.title,!0,`text`),t+=h(`Subtitle`,`subtitle`,e.subtitle,!0,`text`),t+=h(`Authors`,`authors`,e.authors,!1,`csv`),t+=h(`Language`,`language`,e.language,!1,`text`),t+=h(`Publisher`,`publisher`,e.publisher,!0,`text`),t+=h(`Description`,`description`,e.description,!0,`textarea`),t+=h(`Source URL`,`source_url`,e.source_url,!0,`text`),t+=h(`License`,`license`,e.license,!0,`text`),t+=h(`Edition`,`edition`,e.edition,!0,`text`),t+=h(`Word Count`,`word_count`,e.word_count,!0,`number`),t+=h(`Reading Time (min)`,`reading_time_mins`,e.reading_time_mins,!0,`number`),t+=Y(`Genres`,`genres`,e.genres),t+=Y(`Tags`,`tags`,e.tags),t+=`<h3>Identifiers</h3><div id="idList">`;let n=e.identifiers||[];t+=n.map((e,t)=>`
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${f(e.id_type||``)}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${f(e.value||``)}" /></div>
      <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">${l(`X`,14)}</button>
    </div>`).join(``),t+=`</div><button class="btn btn-secondary" data-add-id="1" style="font-size:0.85rem;margin-top:0.3rem">+ Add Identifier</button>`,t+=`<h3>Series</h3>`,e.series?(t+=h(`Series Title`,`series_title`,e.series.title,!0,`text`),t+=h(`Series Position`,`series_pos`,e.series.position,!0,`text`),t+=h(`Series Arc`,`series_arc`,e.series.arc,!0,`text`)):t+=`<p style="color:#888;font-size:0.9rem">No series info</p>`,V.innerHTML=t,V.innerHTML=t}function Y(e,t,n){return`<div class="field"><label>${e}</label>
    <div class="tag-list" id="tags_${t}">${(Array.isArray(n)?n:[]).map(e=>`<span class="tag"><span class="tag-text">${f(e)}</span> <span class="tag-remove" data-tag-id="${t}">${l(`X`,12)}</span></span>`).join(``)}</div>
    <div class="tag-input"><input type="text" id="new_${t}" placeholder="Add ${e.toLowerCase()}" />
    <button data-tag-add="${t}">Add</button></div>
  </div>`}function pe(){let e=w.get();if(!e||e.length===0){U.innerHTML=`<p style='color:#888'>No extra data</p>`;return}U.innerHTML=`<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${p(e.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${_e(e)}</pre>
  </details>`}function me(){let e=structuredClone(E.get());Z(e,`title`,`title`),Z(e,`subtitle`,`subtitle`),e.authors=(document.getElementById(`mf_authors`)?.value||``).split(`,`).map(e=>e.trim()).filter(Boolean),e.language=document.getElementById(`mf_language`)?.value||`en`,Z(e,`publisher`,`publisher`),Z(e,`description`,`description`),Z(e,`source_url`,`source_url`),Z(e,`license`,`license`),Z(e,`edition`,`edition`),X(e,`word_count`,document.getElementById(`mf_word_count`)?.value),X(e,`reading_time_mins`,document.getElementById(`mf_reading_time_mins`)?.value),e.genres=Q(`genres`),e.tags=Q(`tags`);let t=document.querySelectorAll(`#idList > div`),n=[];for(let e of t){let t=e.querySelector(`.id-type`)?.value?.trim(),r=e.querySelector(`.id-value`)?.value?.trim();t&&r&&n.push({id_type:t,value:r})}e.identifiers=n.length>0?n:void 0;let r=document.getElementById(`mf_series_title`)?.value?.trim();return e.series=r?{title:r,position:document.getElementById(`mf_series_pos`)?.value?.trim()||void 0,arc:document.getElementById(`mf_series_arc`)?.value?.trim()||void 0}:void 0,e}function X(e,t,n){let r=parseInt(n,10);if(isNaN(r)){delete e[t];return}e[t]=r}function Z(e,t,n){let r=document.getElementById(`mf_${n}`)?.value?.trim(),i=E.get()?.[t];if(!(i instanceof Map||i&&typeof i==`object`&&!Array.isArray(i))){ne(e,t,r);return}let a=i instanceof Map?[...i.keys()]:Object.keys(i),o={};for(let e of a.slice(1)){let t=document.getElementById(`mf_${n}__${e}`)?.value?.trim();t&&(o[e]=t)}if(!r&&Object.keys(o).length===0){delete e[t];return}if(i instanceof Map){let n=new Map;r&&n.set(a[0],r);for(let[e,t]of Object.entries(o))n.set(e,t);e[t]=n}else{let n={};r&&(n[a[0]]=r),Object.assign(n,o),e[t]=n}}function Q(e){let t=document.getElementById(`tags_`+e);if(!t)return;let n=[];for(let e of t.querySelectorAll(`.tag`)){let t=e.querySelector(`.tag-text`)?.textContent?.trim()||e.textContent.trim();t&&n.push(t)}return n.length>0?n:void 0}function he(){if(!_){$(`error`,`No file loaded to save`);return}$(`loading`,`Saving changes...`);try{let e=me(),t=T.get().map(e=>({tag:e.tag,data:new Uint8Array(e.data),compression:e.compression,content_type_kind:e.content_type_kind,content_type_value:e.content_type_value,alt_text:e.alt_text,font_embedding:e.font_embedding,font_license_url:e.font_license_url})),n=w.get(),r=a({chunks:t,meta:e,extra:n?.length?new Uint8Array(n):void 0,language:e.language||`en`,auto_sidx:!0}),i=y.get().replace(/\.hzo$/i,`_edited.hzo`);ee(r,i),$(`success`,`File saved successfully as ${i}`)}catch(e){console.error(`Error saving file:`,e),$(`error`,`Failed to save file: ${e.message||String(e)}`)}}function ge(){let e=E.get();e?(C.set(JSON.parse(JSON.stringify(e))),J(),$(`success`,`Metadata reverted to original`)):$(`error`,`No original metadata to revert to`)}function $(e,t){D.set(!0),O.set(e),k.set(t),(e===`success`||e===`loading`)&&setTimeout(()=>{O.get()===e&&D.set(!1)},5e3)}function _e(e){let t=new Uint8Array(e||[]),n=``;for(let e=0;e<Math.min(t.length,512);e++)e>0&&e%32==0&&(n+=`
`),n+=t[e].toString(16).padStart(2,`0`)+` `;return t.length>512&&(n+=`\n... (${t.length-512} more bytes)`),n}
//# sourceMappingURL=inspect-BT8qJ_8Z.js.map