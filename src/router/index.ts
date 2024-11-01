import { createRouter,createMemoryHistory } from 'vue-router'
import msg from '../views/msg/index.vue'

const routes = [
  {
    path: '/',
    name: 'msg',
    component: msg
  },
  {
    path: '/about',
    name: 'about',
    component: () => import('../views/about.vue')
  },
  {
    path: '/testVideo',
    name: 'testVideo',
    component: () => import('../views/videoTest/index.vue')
  }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes
})

export default router