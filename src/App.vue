
<template>
  <div id="app">
    <header>
      <h1>WantFastDns</h1>
      <p>DNS 测速工具</p>
    </header>

    <main>
      <DnsSpeed />
    </main>

    <footer>
      <p>© 2025 WantFastDns</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import DnsSpeed from './components/DnsSpeed.vue';
</script>

<style>
#app {
  font-family: 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  color: #333;
  text-align: center;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* 隐藏滚动条，但内容仍可滚动 */
html, body {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow-y: auto; /* 允许纵向滚动 */
  scrollbar-width: thin; /* Firefox 可选美化 */
}

body::-webkit-scrollbar {
  width: 0; /* Chrome/Edge/Safari 隐藏滚动条 */
}


header {
  background-color: #4f46e5;
  color: white;
  padding: 20px 0;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

header h1 {
  margin: 0;
  font-size: 2rem;
}

header p {
  margin: 5px 0 0;
}

main {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 20px;
}

footer {
  padding: 10px 0;
  font-size: 12px;
  color: #888;
}
</style>
style>
