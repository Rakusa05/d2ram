<script lang="ts">
  let {
    open = $bindable(),
    btnActionName = "Ok",
    onAccept,
    children,
  } = $props<{
    open: boolean;
    btnActionName?: string;
    onAccept?: () => void | Promise<void>;
    children(): any;
  }>();

  const close = () => (open = false);
  const accept = async () => {
    if (onAccept) await onAccept();
    close();
  };
</script>

<div
  class={`absolute bg-[oklch(0.1_0_0_/0.6)] w-screen h-screen z-20 ${open ? "scale-100" : "scale-0"} scale-0 flex items-center justify-center`}
  onpointerdown={close}
>
  <div
    class="bg-neutral-900 w-100 h-auto flex flex-col justify-evenly items-center p-5 rounded-2xl z-30"
    onpointerdown={(e) => {
      e.stopPropagation();
    }}
  >
    {@render children()}

    <div class="pt-10 flex space-x-6">
      <button
        class="bg-red-600 border border-black hover:bg-red-500 rounded-xl px-5 py-1 font-bold"
        onclick={accept}>{btnActionName}</button
      >
      <button
        class="bg-red-600 border border-black hover:bg-red-500 rounded-xl px-5 py-1 font-bold"
        onclick={close}>Cancel</button
      >
    </div>
  </div>
</div>
