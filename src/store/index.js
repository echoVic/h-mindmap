import { atom } from 'recoil';

export const nodesState = atom({
  key: 'nodesState',
  default: [],
});

export const linksState = atom({
  key: 'linksState',
  default: [],
});