import{c as e,d as t,g as n,h as r,i,l as a,m as o,n as s,p as ee,s as te,t as ne}from"./honzo_wasm-D0D7BzBM.js";import{t as c}from"./esc-Bzx7pNv8.js";import{n as l,t as re}from"./download-DA2FER3_.js";var u=!1,d=null,f=n(r(`inspect`,`fileLoaded`),!1),p=n(r(`inspect`,`fileName`),``),m=n(r(`inspect`,`fileSize`),0),h=n(r(`inspect`,`fileInfoData`),null),g=n(r(`inspect`,`tocData`),[]),_=n(r(`inspect`,`metaData`),null),v=n(r(`inspect`,`extraData`),null),y=n(r(`inspect`,`chunksData`),[]),b=n(r(`inspect`,`originalMeta`),null),x=n(r(`inspect`,`statusVisible`),!1),S=n(r(`inspect`,`statusKind`),``),C=n(r(`inspect`,`statusMessage`),``),w=o(r(`inspect`,`statusClass`),()=>`status${x.get()?` active ${S.get()}`:``}`),T=o(r(`inspect`,`filePanelClass`),()=>`panel${f.get()?` visible`:``}`),E=o(r(`inspect`,`tocPanelClass`),()=>`panel${f.get()?` visible`:``}`),D=o(r(`inspect`,`metaPanelClass`),()=>`panel${f.get()?` visible`:``}`),O=o(r(`inspect`,`extraPanelClass`),()=>`panel${f.get()&&v.get()?.length>0?` visible`:``}`),k=o(r(`inspect`,`saveDisabled`),()=>!f.get()),A=o(r(`inspect`,`revertDisabled`),()=>!f.get()),j=document.getElementById(`dropZone`),M=document.getElementById(`fileInput`),ie=document.getElementById(`status`),ae=document.getElementById(`statusText`),oe=document.getElementById(`filePanel`),N=document.getElementById(`fileInfo`),P=document.getElementById(`tocPanel`),F=document.getElementById(`chunkCount`),I=document.getElementById(`tocBody`),L=document.getElementById(`metaPanel`),R=document.getElementById(`metaFields`),z=document.getElementById(`extraPanel`),B=document.getElementById(`extraInfo`),V=document.getElementById(`revertBtn`),H=document.getElementById(`saveBtn`);a(ie,w),ee(ae,C),a(oe,T),a(P,E),a(L,D),a(z,O);var U=(e,t)=>{let n=()=>e.disabled=t.get();n(),t.subscribe?.(n)??(t._onSet=n)};U(H,k),U(V,A),t(j,`click`,()=>M.click()),t(j,`dragover`,e=>{e.preventDefault(),j.classList.add(`dragover`)}),t(j,`dragleave`,()=>j.classList.remove(`dragover`)),t(j,`drop`,e=>{e.preventDefault(),j.classList.remove(`dragover`),G(e.dataTransfer?.files?.[0])}),t(M,`change`,e=>{e.target.files?.[0]&&G(e.target.files[0])}),t(H,`click`,de),t(V,`click`,fe),e({icons:te}),t(L,`click`,e=>{if(e.target.closest(`[data-add-id]`)){let e=document.getElementById(`idList`);e&&e.insertAdjacentHTML(`beforeend`,`
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
      </div>`)}let t=e.target.closest(`[data-remove]`);t&&t.closest(`.field`)?.remove();let n=e.target.closest(`[data-tag-add]`);if(n){let e=n.dataset.tagAdd,t=document.getElementById(`new_${e}`),r=t?.value?.trim();if(r){let n=document.getElementById(`tags_${e}`);n&&n.insertAdjacentHTML(`beforeend`,`<span class="tag"><span class="tag-text">${c(r)}</span> <span class="tag-remove" data-tag-id="${e}">×</span></span>`),t.value=``}}let r=e.target.closest(`.tag-remove`);r&&r.parentElement.remove()});async function W(){u||=(await s(),!0)}async function G(e){if(!e){$(`error`,`No file selected`);return}if(!e.name.endsWith(`.hzo`)){$(`error`,`Please select a .hzo file`);return}$(`loading`,`Loading ${e.name}...`);try{await W();let t=await e.arrayBuffer();d=new ne(new Uint8Array(t),1),p.set(e.name),m.set(t.byteLength);let n=d.get_extra(),r=K(d),i=d.get_meta_parsed();h.set({versionMajor:d.version_major(),versionMinor:d.version_minor(),minVer:d.min_reader_version(),flags:d.flags(),chunkCount:d.chunk_count(),tocSize:d.toc_size(),dataSize:d.data_size(),extraSize:d.extra_size(),metaSize:d.meta_size()}),g.set(d.get_toc()),_.set(i),b.set(JSON.parse(JSON.stringify(i))),v.set(n),y.set(r),se(),ce(),q(),le(),f.set(!0),$(`success`,`Successfully loaded: ${e.name} (${l(t.byteLength)})`)}catch(e){console.error(`Error loading file:`,e),$(`error`,`Failed to load file: ${e.message||String(e)}`)}}function K(e){return e.get_toc().map((t,n)=>({tag:t.chunk_type,data:Array.from(e.get_chunk(n)||new Uint8Array),compression:t.compression,content_type_kind:t.content_type_kind,content_type_value:t.content_type_value,cover_type:t.cover_type,alt_text:t.alt_text||null,font_embedding:t.font_embedding,font_license_url:t.font_license_url||null}))}function se(){let e=h.get();if(!e||!d)return;let t=d.layout_mode_name(),n=d.compression_name(),r=(e,t)=>`<span class="flag-badge ${e?`on`:`off`}">${e?`Yes`:`No`}</span>`;N.innerHTML=`
    <div class="info-grid">
      <div class="info-item">
        <span class="label">File Size</span>
        <div class="value">${l(m.get())}</div>
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
        <div class="value">${l(e.tocSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Data Size</span>
        <div class="value">${l(e.dataSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Extra Data Size</span>
        <div class="value">${l(e.extraSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Metadata Size</span>
        <div class="value">${l(e.metaSize)}</div>
      </div>
      <div class="info-item" style="grid-column: 1 / -1">
        <span class="label">Features</span>
        <div class="value features-grid">
          <div>
            <span>Search Index:</span>
            ${r(d.has_sidx(),`Search Index`)}
          </div>
          <div>
            <span>DRM:</span>
            ${r(d.has_drm(),`DRM`)}
          </div>
          <div>
            <span>Annotations:</span>
            ${r(d.has_annotations(),`Annotations`)}
          </div>
          <div>
            <span>Sync:</span>
            ${r(d.has_sync(),`Sync`)}
          </div>
        </div>
      </div>
    </div>
  `}function ce(){let e=g.get();F.textContent=`(${h.get()?.chunkCount||0} total)`,I.innerHTML=e.map((e,t)=>{let n=typeof e.chunk_type==`string`?e.chunk_type:new TextDecoder().decode(new Uint8Array(e.chunk_type)),r=d.compression_name_for_chunk(t),i=d.content_type_name_for_chunk(t);return`<tr>
      <td>${t}</td>
      <td><strong>${c(n)}</strong></td>
      <td>${l(Number(e.size_compressed))}</td>
      <td>${l(Number(e.size_raw))}</td>
      <td>${r}</td>
      <td>${i}</td>
      <td>0x${e.flags.toString(16).padStart(4,`0`)}</td>
    </tr>`}).join(``)}function q(){let e=_.get();if(!e||typeof e!=`object`){R.innerHTML=`<p style='color:#888'>No metadata</p>`;return}let t=``;t+=J(`Title`,`title`,e.title,!0,`text`),t+=J(`Subtitle`,`subtitle`,e.subtitle,!0,`text`),t+=J(`Authors`,`authors`,e.authors,!1,`csv`),t+=J(`Language`,`language`,e.language,!1,`text`),t+=J(`Publisher`,`publisher`,e.publisher,!0,`text`),t+=J(`Description`,`description`,e.description,!0,`textarea`),t+=J(`Source URL`,`source_url`,e.source_url,!0,`text`),t+=J(`License`,`license`,e.license,!0,`text`),t+=J(`Edition`,`edition`,e.edition,!0,`text`),t+=J(`Word Count`,`word_count`,e.word_count,!0,`number`),t+=J(`Reading Time (min)`,`reading_time_mins`,e.reading_time_mins,!0,`number`),t+=Y(`Genres`,`genres`,e.genres),t+=Y(`Tags`,`tags`,e.tags),t+=`<h3>Identifiers</h3><div id="idList">`;let n=e.identifiers||[];t+=n.map((e,t)=>`
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${c(e.id_type||``)}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${c(e.value||``)}" /></div>
      <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
    </div>`).join(``),t+=`</div><button class="btn btn-secondary" data-add-id="1" style="font-size:0.85rem;margin-top:0.3rem">+ Add Identifier</button>`,t+=`<h3>Series</h3>`,e.series?(t+=J(`Series Title`,`series_title`,e.series.title,!0,`text`),t+=J(`Series Position`,`series_pos`,e.series.position,!0,`text`),t+=J(`Series Arc`,`series_arc`,e.series.arc,!0,`text`)):t+=`<p style="color:#888;font-size:0.9rem">No series info</p>`,R.innerHTML=t,R.innerHTML=t}function J(e,t,n,r,i){let a=n??``,o=typeof a==`object`&&a?Object.values(a)[0]||``:String(a),s=i===`textarea`?`<textarea id="mf_${t}">${c(o)}</textarea>`:i===`csv`?`<input type="text" id="mf_${t}" value="${c(Array.isArray(a)?a.join(`, `):o)}" />`:`<input type="${i}" id="mf_${t}" value="${c(o)}" />`;return`<div class="field"><label for="mf_${t}">${e}${r?``:` *`}</label>${s}</div>`}function Y(e,t,n){return`<div class="field"><label>${e}</label>
    <div class="tag-list" id="tags_${t}">${(Array.isArray(n)?n:[]).map(e=>`<span class="tag"><span class="tag-text">${c(e)}</span> <span class="tag-remove" data-tag-id="${t}">×</span></span>`).join(``)}</div>
    <div class="tag-input"><input type="text" id="new_${t}" placeholder="Add ${e.toLowerCase()}" />
    <button data-tag-add="${t}">Add</button></div>
  </div>`}function le(){let e=v.get();if(!e||e.length===0){B.innerHTML=`<p style='color:#888'>No extra data</p>`;return}B.innerHTML=`<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${l(e.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${pe(e)}</pre>
  </details>`}function ue(){let e=JSON.parse(JSON.stringify(b.get()));X(e,`title`,document.getElementById(`mf_title`)?.value),X(e,`subtitle`,document.getElementById(`mf_subtitle`)?.value),e.authors=(document.getElementById(`mf_authors`)?.value||``).split(`,`).map(e=>e.trim()).filter(Boolean),e.language=document.getElementById(`mf_language`)?.value||`en`,X(e,`publisher`,document.getElementById(`mf_publisher`)?.value),X(e,`description`,document.getElementById(`mf_description`)?.value),X(e,`source_url`,document.getElementById(`mf_source_url`)?.value),X(e,`license`,document.getElementById(`mf_license`)?.value),X(e,`edition`,document.getElementById(`mf_edition`)?.value),Z(e,`word_count`,document.getElementById(`mf_word_count`)?.value),Z(e,`reading_time_mins`,document.getElementById(`mf_reading_time_mins`)?.value),e.genres=Q(`genres`),e.tags=Q(`tags`);let t=document.querySelectorAll(`#idList > div`),n=[];for(let e of t){let t=e.querySelector(`.id-type`)?.value?.trim(),r=e.querySelector(`.id-value`)?.value?.trim();t&&r&&n.push({id_type:t,value:r})}e.identifiers=n.length>0?n:void 0;let r=document.getElementById(`mf_series_title`)?.value?.trim();return r?e.series={title:r,position:document.getElementById(`mf_series_pos`)?.value?.trim()||void 0,arc:document.getElementById(`mf_series_arc`)?.value?.trim()||void 0}:e.series=void 0,e}function X(e,t,n){if(!n||!n.trim()){delete e[t];return}let r=n.trim();if(e[t]&&typeof e[t]==`object`&&!Array.isArray(e[t])){e[t]={...e[t]};let n=Object.keys(e[t]);n.length>0?e[t][n[0]]=r:e[t]={en:r}}else e[t]=r}function Z(e,t,n){let r=parseInt(n,10);if(isNaN(r)){delete e[t];return}e[t]=r}function Q(e){let t=document.getElementById(`tags_`+e);if(!t)return;let n=[];for(let e of t.querySelectorAll(`.tag`)){let t=e.querySelector(`.tag-text`)?.textContent?.trim()||e.textContent.replace(`×`,``).trim();t&&n.push(t)}return n.length>0?n:void 0}function de(){if(!d){$(`error`,`No file loaded to save`);return}$(`loading`,`Saving changes...`);try{let e=ue(),t=y.get().map(e=>({tag:e.tag,data:new Uint8Array(e.data),compression:e.compression,content_type_kind:e.content_type_kind,content_type_value:e.content_type_value,alt_text:e.alt_text,font_embedding:e.font_embedding,font_license_url:e.font_license_url})),n=v.get(),r=i({chunks:t,meta:e,extra:n?.length?new Uint8Array(n):void 0,language:e.language||`en`,auto_sidx:!0}),a=p.get().replace(/\.hzo$/i,`_edited.hzo`);re(r,a),$(`success`,`File saved successfully as ${a}`)}catch(e){console.error(`Error saving file:`,e),$(`error`,`Failed to save file: ${e.message||String(e)}`)}}function fe(){let e=b.get();e?(_.set(JSON.parse(JSON.stringify(e))),q(),$(`success`,`Metadata reverted to original`)):$(`error`,`No original metadata to revert to`)}function $(e,t){x.set(!0),S.set(e),C.set(t),(e===`success`||e===`loading`)&&setTimeout(()=>{S.get()===e&&x.set(!1)},5e3)}function pe(e){let t=new Uint8Array(e||[]),n=``;for(let e=0;e<Math.min(t.length,512);e++)e>0&&e%32==0&&(n+=`
`),n+=t[e].toString(16).padStart(2,`0`)+` `;return t.length>512&&(n+=`\n... (${t.length-512} more bytes)`),n}
//# sourceMappingURL=inspect-DQVSNEUO.js.map