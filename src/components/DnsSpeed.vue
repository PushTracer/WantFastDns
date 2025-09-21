<template>
  <div class="dns-speed">
    <h2>DNS 测速工具</h2>

    <div class="controls">
      <label>
        要测速的域名：
        <input v-model="domain" placeholder="www.google.com" />
      </label>

      <label>
        DNS 服务器 (逗号分隔)：
        <input v-model="dnsInput" placeholder="8.8.8.8:53,1.1.1.1:53" />
      </label>

      <button @click="runTest" :disabled="loading">
        {{ loading ? "测速中..." : "开始测速" }}
      </button>
    </div>

    <table v-if="results.length" border="1" cellpadding="5" style="margin-top: 20px;">
      <thead>
        <tr>
          <th>DNS 服务器</th>
          <th>响应时间 (ms)</th>
          <th>状态</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="r in sortedResults" :key="r.dns">
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

// 控件状态
const results = ref<DnsResult[]>([]);
const loading = ref(false);
const domain = ref("www.google.com");
const dnsInput = ref("8.8.8.8:53,1.1.1.1:53,114.114.114.114:53");

// 排序结果，最快响应在前，超时放后
const sortedResults = computed(() => {
  return results.value
    .slice()
    .sort((a, b) => {
      if (a.time_ms === null) return 1;
      if (b.time_ms === null) return -1;
      return a.time_ms - b.time_ms;
    });
});

async function runTest() {
  loading.value = true;
  results.value = [];

  // 将输入的 DNS 字符串转成数组
  const dnsServers = dnsInput.value.split(",").map(s => s.trim()).filter(Boolean);

  try {
    // 调用 Tauri 后端，每个 DNS 并发测速
    const res = await invoke<DnsResult[]>("dns_speed_test", {
      dnsServers,
      domain: domain.value
    });
    results.value = res;
  } catch (e) {
    console.error("调用 Tauri 后端失败", e);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.dns-speed {
  padding: 20px;
  max-width: 600px;
}

.controls {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.controls input {
  width: 100%;
  padding: 5px;
}

.controls button {
  width: 120px;
  padding: 8px 16px;
  cursor: pointer;
}
</style>
