import { invoke, isTauri } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import './style.css';
const $ = (id) => document.getElementById(id);
const controls = ['length', 'lowercase', 'uppercase', 'digits', 'symbols', 'exclude'];
let revision = 0;
let copying = false;
function invalidate() { revision++; $('password').value = ''; $('copy').disabled = true; $('status').textContent = ''; }
async function generate() {
  invalidate();
  const current = revision;
  $('generate').disabled = true;
  try {
    if (!isTauri()) throw new Error('Bitte die Desktop-App öffnen.');
    const password = await invoke('generate_password', { options: {
      length: Number($('length').value), lowercase: $('lowercase').checked,
      uppercase: $('uppercase').checked, digits: $('digits').checked,
      symbols: $('symbols').checked, exclude_similar: $('exclude').checked
    }});
    if (current !== revision) return;
    $('password').value = password;
    $('copy').disabled = copying;
  } catch (error) { if (current === revision) $('status').textContent = String(error); }
  finally { $('generate').disabled = false; }
}
for (const id of controls) $(id).addEventListener('input', () => {
  invalidate(); $('length-value').value = $('length').value;
});
$('generate').addEventListener('click', generate);
$('copy').addEventListener('click', async () => {
  const password = $('password').value;
  if (!password || copying) return;
  const current = revision;
  copying = true; $('copy').disabled = true;
  try {
    await writeText(password);
    if (current === revision) $('status').textContent = 'Kopiert.';
  } catch { if (current === revision) $('status').textContent = 'Kopieren fehlgeschlagen.'; }
  finally { copying = false; $('copy').disabled = !$('password').value; }
});
generate();
