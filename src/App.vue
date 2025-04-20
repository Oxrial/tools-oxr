<template>
	<div class="main">
		<div class="operation-bar">
			<ElButton @click="selectFolder" type="primary" plain> 选择文件夹 </ElButton>
			<ElButtonGroup>
				<el-input
					v-model="folderPath"
					placeholder="键入地址"
					@focus="() => (disabledScan = true)"
					@blur="() => (disabledScan = false)"
				>
				</el-input>
				<ElButton :disabled="!sortedFiles.length" type="success" plain @click="confirmAndMerge"
					>生成 ({{ submit.length }}/{{ sortedFiles.length }})</ElButton
				>
			</ElButtonGroup>
		</div>
		<div>
			<ElCard class="flv-list">
				<VueDraggable v-model="sortedFiles" ghostClass="ghost" target="tbody" :animation="150">
					<el-table :data="sortedFiles" :cell-class-name="renderCellClass" height="calc(100vh - 7rem)">
						<el-table-column label="文件名" prop="name" :width="getColumnWidth('name', sortedFiles)" />
						<el-table-column show-overflow-tooltip="" label="全路径" prop="id" />
						<el-table-column label="操作" width="70">
							<template #default="scope">
								<el-button link type="primary" size="small" @click="handleDelete(scope.row)">
									{{ scope.row.delete ? '撤销' : '屏蔽' }}
								</el-button>
							</template>
						</el-table-column>
					</el-table>
				</VueDraggable>
			</ElCard>
		</div>
	</div>
</template>

<script setup>
import { ref, watch, watchEffect } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { VueDraggable } from 'vue-draggable-plus'
import { getColumnWidth } from './index'
import { join } from 'path-browserify'

const folderPath = ref('')
const files = ref([])
const sortedFiles = ref([])
const disabledScan = ref(false)
const selectFolder = async () => {
	try {
		folderPath.value = await invoke('select_folder')
		// scanFlvFiles(folderPath.value)
	} catch (error) {
		alert(`Error: ${error}`)
	}
}
const scanFlvFiles = async (folder) => {
	if (disabledScan.value || !folderPath.value.length) return
	files.value = await invoke('scan_flv_files', { path: folder })
	sortedFiles.value = files.value.map((f) => ({
		name: f.substring(f.lastIndexOf('/') + 1),
		id: join(folder, f),
		delete: false
	}))
	console.log('🚀 ~ selectFolder ~ files:', sortedFiles)
}

const handleDelete = (row) => {
	row.delete = !row.delete
}
const renderCellClass = (data) => {
	return (data.columnIndex === 0 ? 'flv-name ' : '') + (data.row.delete && data.columnIndex !== 2 ? ' delete' : '')
}
watchEffect(() => {
	scanFlvFiles(folderPath.value)
})
const submit = computed(() => sortedFiles.value.filter((s) => !s.delete).map((s) => s.id))
const confirmAndMerge = async () => {
	try {
		await invoke('generate_filelist_and_merge', {
			files: submit.value,
			folderPath: folderPath.value
		})
		alert('视频合并完成！')
	} catch (error) {
		alert(`Error: ${error}`)
	}
}
</script>
<style scoped lang="scss">
.main {
	padding: 20px 20px 0;
}
.operation-bar {
	width: 100%;
	display: flex;
	> :last-child {
		flex: 1;
		display: flex;
		> :last-child {
			width: 30%;
			&.is-disabled {
				background-color: #0001;
			}
		}
	}
}
:deep(.el-input-group__prepend) .el-button {
	all: unset;
}
:deep(.delete) {
	filter: brightness(0.96) blur(1px);
}
:deep(.flv-name) {
	width: fit-content;
}
</style>
<style lang="scss">
.ghost {
	background: rgb(219, 219, 219) !important;
}
</style>
