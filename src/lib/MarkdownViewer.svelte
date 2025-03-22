<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import { open } from "@tauri-apps/api/shell";

	// 儲存要編輯的檔案路徑
	let editPath: string = "";
	// 儲存轉換後的HTML內容
	let htmlContent: string = "";

	/**
	 * 載入並渲染Markdown檔案
	 * @param filePath - Markdown檔案的路徑
	 */
	async function loadMarkdown(filePath: string) {
		try {
			const html = await invoke("open_markdown", { path: filePath });
			htmlContent = html as string;
		} catch (error) {
			console.error("Error loading Markdown file:", error);
		}
	}

	/**
	 * 在編輯器中開啟當前檔案
	 */
	async function openInEditor() {
		if (editPath) {
			await open(editPath, "code");
		} else {
			console.error("No file path available to open in editor.");
		}
	}

	// 元件掛載時，獲取Markdown檔案路徑並載入內容
	onMount(() => {
		invoke("send_markdown_path")
			.then((path: string) => {
				editPath = path;
				loadMarkdown(path);
			})
			.catch((error) => {
				console.error("Error receiving Markdown file path:", error);
			});
	});

	// 元件掛載時，設置視窗並攔截連結
	onMount(async () => {
		setTimeout(() => {
			setupWindow();
			interceptLinks();
		}, 300);
	});

	/**
	 * 設置並顯示應用程式視窗
	 * 參考: https://github.com/tauri-apps/tauri/issues/5170
	 */
	async function setupWindow() {
		const appWindow = (await import("@tauri-apps/api/window")).appWindow;
		appWindow.show();
	}

	/**
	 * 攔截頁面中的連結點擊事件，用系統瀏覽器開啟外部連結
	 */
	function interceptLinks() {
		document.addEventListener("click", async (event) => {
			let target = event.target as HTMLElement;

			// 尋找最近的<a>標籤
			while (target && target.tagName !== "A" && target !== document.body) {
				target = target.parentElement;
			}

			if (
				target &&
				target.tagName === "A" &&
				(target as HTMLAnchorElement).href
			) {
				const anchor = target as HTMLAnchorElement;

				// 若不是頁內連結(#)，則用系統瀏覽器開啟
				if (!anchor.href.startsWith("#")) {
					event.preventDefault();
					await open(anchor.href);
				}
			}
		});
	}
</script>

<!-- 當沒有內容時顯示提示訊息 -->
{#if !htmlContent}
	<div class="message">
		<p>
			Open a Markdown file by right-clicking and selecting 'Open with...'
			&rightarrow; 'Markdown Viewer'
		</p>
	</div>
<!-- 顯示Markdown轉換後的HTML內容 -->
{:else}
	<article
		contenteditable="false"
		class="markdown-body"
		bind:innerHTML={htmlContent}>
	</article>
{/if}

<style>
	/* 根元素的動畫變數 */
	:root {
		--animation: cubic-bezier(0.05, 0.95, 0.05, 0.95);
	}

	/* Markdown內容的樣式 */
	.markdown-body {
		box-sizing: border-box;
		min-width: 200px;
		max-width: 980px;
		margin: 0 auto;
		padding: 45px;
	}

	/* 提示訊息的樣式 */
	.message {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		user-select: none;
		font-family:
			Segoe UI,
			Helvetica Neue,
			Helvetica,
			Arial,
			sans-serif;
		height: 90vh;
	}

	/* 深色模式下的提示訊息顏色 */
	@media (prefers-color-scheme: dark) {
		.message {
			color: #ffffffaa;
		}
	}

	/* 淺色模式下的提示訊息顏色 */
	@media (prefers-color-scheme: light) {
		.message {
			color: #000000aa;
		}
	}
</style>
