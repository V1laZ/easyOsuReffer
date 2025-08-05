<template>
  <div>
    <OAuthCallback v-if="modalsState.showOAuthCallback" />
  </div>
</template>

<script setup lang="ts">
import OAuthCallback from './OAuthCallback.vue';
import { modalsState } from '../../stores/global';

</script>