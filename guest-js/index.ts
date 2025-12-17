import { invoke } from '@tauri-apps/api/core';

export async function apply_liquid_glass(
	value: string
): Promise<string | null> {
	return await invoke<{ value?: string }>(
		'plugin:liquid-glass|apply_liquid_glass',
		{
			payload: {
				value,
			},
		}
	).then((r) => (r.value ? r.value : null));
}

export async function set_liquid_glass_style(
	value: string
): Promise<string | null> {
	return await invoke<{ value?: string }>(
		'plugin:liquid-glass|set_liquid_glass_style',
		{
			payload: {
				value,
			},
		}
	).then((r) => (r.value ? r.value : null));
}
