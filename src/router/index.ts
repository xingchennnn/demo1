import { createRouter,createMemoryHistory } from 'vue-router'
import msg from '../views/msg/index.vue'
import About from '../views/About.vue'

const routes = [
  {
    path: '/',
    name: 'msg',
    component: msg
  },
  {
    path: '/about',
    name: 'about',
    component: About
  }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes
})

export default router