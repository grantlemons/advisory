<script lang="ts">
    export let color: string = '#000000';
    export let thickness: number = 1.5;

    function setCssProps(node: HTMLDivElement) {
        node.style.setProperty(`--color`, color);
        node.style.setProperty(`--thickness`, thickness + 'px');
        node.style.setProperty(`--radius`, thickness / 2 + 'px');
    }
</script>

<div class="hrule" use:setCssProps />

<style>
    div {
        background-color: var(--color);
        height: var(--thickness);
        width: 102%;
        border-radius: var(--radius);
    }
</style>
