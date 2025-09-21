<template>
  <div id="app">
    <header>
      <h1>WantFastDns - DNS 测速工具</h1>
    </header>

    <main>
      <!-- 引入刚写好的 DnsSpeed 组件 -->
      <DnsSpeed />
    </main>

    <footer>
      <p>© 2025 WantFastDns</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
// 引入组件
import DnsSpeed from './components/DnsSpeed.vue';
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 40px;
}

header {
  margin-bottom: 20px;
}

main {
  display: flex;
  justify-content: center;
}

footer {
  margin-top: 40px;
  font-size: 12px;
  color: #888;
}
</style>
/style>
