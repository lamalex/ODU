import { mount } from "@vue/test-utils";
import About from "@/views/About.vue";

it("lists everyone", () => {
  const wrapper = mount(About);
  const list = wrapper.findAll("li");
  expect(list.length).toBe(4);
});
