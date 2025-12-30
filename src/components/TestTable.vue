<template>

  <div class="q-pa-md">
    <!-- Search Box -->
    <div class="row justify-start q-mb-md">
      <q-input
        v-model="searchQuery"
        outlined
        dense
        placeholder="Search treats by name, calories, fat, carbs, protein..."
        style="width: 400px"
      >
        <template v-slot:prepend>
          <q-icon name="search" />
        </template>
        <template v-slot:append v-if="searchQuery">
          <q-icon name="close" @click="searchQuery = ''" class="cursor-pointer" />
        </template>
      </q-input>
    </div>


    <!-- Table -->
    <q-table
      title=""
      :rows="filteredRows"
      :columns="columns"
      row-key="name"
      flat
      bordered
      separator="cell"
      class="table-striped"
    >
      <!-- Empty State -->
      <template v-slot:no-data>
        <div class="full-width row flex-center q-pa-lg">
          <div class="text-center">
            <q-icon name="search_off" size="48px" color="grey-5" class="q-mb-md" />
            <p class="text-grey">No treats found matching "{{ searchQuery }}"</p>
          </div>
        </div>
      </template>
    </q-table>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const searchQuery = ref('')

const columns = [
  {
    name: 'name',
    required: true,
    label: 'Dessert (100g serving)',
    align: 'left',
    field: (row) => row.name,
    format: (val) => `${val}`,
    sortable: true,
  },
  { name: 'calories', align: 'center', label: 'Calories', field: 'calories', sortable: true },
  { name: 'fat', label: 'Fat (g)', field: 'fat', sortable: true },
  { name: 'carbs', label: 'Carbs (g)', field: 'carbs' },
  { name: 'protein', label: 'Protein (g)', field: 'protein' },
]

const rows = [
  {
    name: 'Frozen Yogurt',
    calories: 159,
    fat: 6.0,
    carbs: 24,
    protein: 4.0,
    sodium: 87,
    calcium: '14%',
    iron: '1%',
  },
  {
    name: 'Ice cream sandwich',
    calories: 237,
    fat: 9.0,
    carbs: 37,
    protein: 4.3,
    sodium: 129,
    calcium: '8%',
    iron: '1%',
  },
  {
    name: 'Eclair',
    calories: 262,
    fat: 16.0,
    carbs: 23,
    protein: 6.0,
    sodium: 337,
    calcium: '6%',
    iron: '7%',
  },
  {
    name: 'Cupcake',
    calories: 305,
    fat: 3.7,
    carbs: 67,
    protein: 4.3,
    sodium: 413,
    calcium: '3%',
    iron: '8%',
  },
  {
    name: 'Gingerbread',
    calories: 356,
    fat: 16.0,
    carbs: 49,
    protein: 3.9,
    sodium: 327,
    calcium: '7%',
    iron: '16%',
  },
  {
    name: 'Jelly bean',
    calories: 375,
    fat: 0.0,
    carbs: 94,
    protein: 0.0,
    sodium: 50,
    calcium: '0%',
    iron: '0%',
  },
  {
    name: 'Lollipop',
    calories: 392,
    fat: 0.2,
    carbs: 98,
    protein: 0,
    sodium: 38,
    calcium: '0%',
    iron: '2%',
  },
  {
    name: 'Honeycomb',
    calories: 408,
    fat: 3.2,
    carbs: 87,
    protein: 6.5,
    sodium: 562,
    calcium: '0%',
    iron: '45%',
  },
  {
    name: 'Donut',
    calories: 452,
    fat: 25.0,
    carbs: 51,
    protein: 4.9,
    sodium: 326,
    calcium: '2%',
    iron: '22%',
  },
  {
    name: 'KitKat',
    calories: 518,
    fat: 26.0,
    carbs: 65,
    protein: 7,
    sodium: 54,
    calcium: '12%',
    iron: '6%',
  },
]

// Filter rows based on search query
const filteredRows = computed(() => {
  if (!searchQuery.value.trim()) {
    return rows
  }

  const query = searchQuery.value.toLowerCase()

  return rows.filter((row) =>
    columns.some((col) => {
      const fieldValue =
        typeof col.field === 'function' ? col.field(row) : row[col.field || col.name]
      return fieldValue && String(fieldValue).toLowerCase().includes(query)
    }),
  )
})
</script>

<style scoped>
.table-striped :deep(tbody tr:nth-child(odd)) {
  background-color: #f5f5f5;
}
</style>
