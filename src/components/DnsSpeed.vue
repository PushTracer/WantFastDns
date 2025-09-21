
<template>
  <div class="dns-speed-card">
    <h2>DNS 测速工具</h2>

    <div class="controls">
      <div class="input-group">
        <label>要测速的域名：</label>
        <input v-model="domain" placeholder="www.google.com" />
      </div>

      <div class="input-group">
        <label>DNS 服务器 (逗号分隔)：</label>
        <input v-model="dnsInput" placeholder="8.8.8.8:53,1.1.1.1:53" />
      </div>

      <button @click="runTest" :disabled="loading">
        {{ loading ? "测速中..." : "开始测速" }}
      </button>
    </div>

    <table v-if="results.length" class="results-table">
      <thead>
        <tr>
          <th>DNS 服务器</th>
          <th>响应时间 (ms)</th>
          <th>状态</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="r in sortedResults" :key="r.dns" :class="{ fastest: r === sortedResults[0] && r.time_ms !== null }">
          <td>{{ r.dns }}</td>
          <td>{{ r.time_ms !== null ? r.time_ms : "-" }}</td>
          <td>{{ r.time_ms !== null ? "成功" : "超时" }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface DnsResult {
  dns: string;
  time_ms: number | null;
}

const results = ref<DnsResult[]>([]);
const loading = ref(false);
const domain = ref("www.baidu.com");
const dnsInput = ref("114.114.114.114:53,114.114.115.115:53,223.5.5.5:53,223.6.6.6:53,180.76.76.76:53,119.29.29.29:53");


const sortedResults = computed(() => {
  return results.value.slice().sort((a, b) => {
    if (a.time_ms === null) return 1;
    if (b.time_ms === null) return -1;
    return a.time_ms - b.time_ms;
  });
});

async function runTest() {
  loading.value = true;
  results.value = [];

  const dnsServers = dnsInput.value.split(",").map(s => s.trim()).filter(Boolean);

  try {
    const res = await invoke<DnsResult[]>("dns_speed_test", { dnsServers, domain: domain.value });
    results.value = res;
  } catch (e) {
    console.error("调用 Tauri 后端失败", e);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.dns-speed-card {
  width: 500px;
  background: white;
  padding: 20px 30px;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  text-align: left;
}

.dns-speed-card h2 {
  margin-top: 0;
  text-align: center;
  color: #4f46e5;
}

.controls {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 20px;
}

.input-group label {
  display: block;
  font-weight: bold;
  margin-bottom: 5px;
}

.input-group input {
  width: 100%;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid #ccc;
  font-size: 14px;
}

button {
  padding: 10px 20px;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: background 0.2s;
}

button:hover:not(:disabled) {
  background-color: #4338ca;
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.results-table {
  width: 100%;
  border-collapse: collapse;
  text-align: center;
}

.results-table th, .results-table td {
  border: 1px solid #ddd;
  padding: 8px;
}

.results-table th {
  background-color: #f3f4f6;
  color: #333;
}

.results-table tr.fastest {
  background-color: #d1fae5;
  font-weight: bold;
}
</style>
/style>
