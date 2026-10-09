<div class="dots">
  <span></span><span></span><span></span>
</div>

<style>
  .dots {
    align-self: flex-start;
    display: flex;
    gap: 5px;
    padding: 10px 14px;
    border-radius: 999px;
    background: rgba(var(--fg-rgb), 0.06);
  }

  span {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--secondary);
    animation: breathe 0.9s ease-in-out infinite alternate;
  }

  span:nth-child(2) {
    animation-delay: 0.22s;
  }

  span:nth-child(3) {
    animation-delay: 0.45s;
  }

  @keyframes breathe {
    from {
      opacity: 0.25;
    }
    to {
      opacity: 1;
    }
  }
</style>
